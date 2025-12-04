//! Database UI Viewer.

use std::collections::BTreeMap;
use std::rc::Rc;

use bytes::Bytes;

use parser::*;

use crate::included_db::INCLUDED_DB;
use crate::{BTreeNodeView, BTreeView, Field, PageElementBuilder, PageLayout, PageView, Part};

#[derive(Debug)]
pub struct Viewer {
    pub included_db: BTreeMap<&'static str, (&'static [u8], &'static [&'static str])>,
    pub pages: Vec<Rc<dyn PageView>>,
    pub btrees: Vec<BTreeView>,
    pub parsed: BTreeMap<&'static str, (Vec<Rc<dyn PageView>>, Vec<BTreeView>)>,
}

pub type Result<T, E = StdError> = std::result::Result<T, E>;

impl Viewer {
    pub fn new() -> Self {
        let dbs: BTreeMap<&'static str, (&'static [u8], &'static [&'static str])> =
            BTreeMap::from_iter(INCLUDED_DB.iter().copied());
        let parsed: BTreeMap<&'static str, (Vec<Rc<dyn PageView>>, Vec<BTreeView>)> =
            BTreeMap::new();

        Self {
            included_db: dbs,
            pages: vec![],
            btrees: vec![],
            parsed,
        }
    }

    pub fn load(&mut self, name: &str) -> Result<(), StdError> {
        if let Some((pages, btrees)) = self.parsed.get(name) {
            self.pages = pages.to_vec();
            self.btrees = btrees.to_vec();
            return Ok(());
        }

        let (name_static, bytes) = match self.included_db.get_key_value(name) {
            Some((&name_static, &(bytes, _))) => (name_static, bytes),
            None => return Err(format!("This db is not included: {}", name).into()),
        };

        self.load_from_bytes(bytes, name_static)
    }

    fn load_from_bytes(
        &mut self,
        bytes: &'static [u8],
        name: &'static str,
    ) -> Result<(), StdError> {
        let reader = Reader::new(bytes)?;
        let size = reader.db_header.page_size as usize;
        let mut pages_map: BTreeMap<usize, Rc<dyn PageView>> = BTreeMap::new();

        // Check if there are freelist pages.
        let freelist_page = reader.db_header.first_free_page_num as usize;
        if freelist_page != 0 {
            if let Ok(page) = reader.get_trunk_freelist_page(freelist_page) {
                Self::load_freelist_pages(page, freelist_page, &mut pages_map, &reader)?;
            };
        }

        let btrees = reader.get_btrees()?;
        let mut view_trees = vec![];
        for tree in btrees {
            let mut view_root = BTreeNodeView::default();
            Self::load_btree_node(tree.root, &mut pages_map, &mut view_root, size);
            view_trees.push(BTreeView {
                ttype: tree.ttype,
                name: tree.name,
                root: view_root,
            })
        }

        let pages: Vec<Rc<dyn PageView>> = pages_map.into_values().collect();

        self.parsed
            .insert(name as &'static str, (pages.clone(), view_trees.clone()));

        self.pages = pages;
        self.btrees = view_trees;
        Ok(())
    }

    pub fn load_from_file(&mut self, bytes: Bytes, name: &str) -> Result<(), StdError> {
        let bytes_static: &'static [u8] = Box::leak(bytes.to_vec().into_boxed_slice());
        let name_static: &'static str = Box::leak(name.to_string().into_boxed_str());

        if let Ok(_) = self.load_from_bytes(bytes_static, name_static) {
            self.included_db.insert(
                name_static,
                (bytes_static, &["Custom local database. Unknown recipe."]),
            );
            Ok(())
        } else {
            Err("Failed to parse database.".into())
        }
    }

    pub fn included_dbnames(&self) -> Vec<String> {
        self.included_db.keys().map(|k| k.to_string()).collect()
    }

    pub fn get_page(&self, id: u32) -> Rc<dyn PageView> {
        self.pages
            .get(id as usize - 1)
            .expect("Page is outside of Viewer range.")
            .clone()
    }

    pub fn get_part(&self, page: &Rc<dyn PageView>, index: usize) -> Rc<dyn Part> {
        page.parts()
            .get(index)
            .expect("Part is outside of Viewer Page range.")
            .clone()
    }

    pub fn get_field(&self, part: &Rc<dyn Part>, index: usize) -> Rc<Field> {
        part.fields()
            .get(index)
            .expect("Fields is outside of Viewer Part range.")
            .clone()
    }

    fn load_btree_node(
        node: BTreeNode,
        pmap: &mut BTreeMap<usize, Rc<dyn PageView>>,
        view_root: &mut BTreeNodeView,
        size: usize,
    ) {
        let page_element = PageLayout::Btree(node.page);
        pmap.insert(
            node.page_num,
            Rc::new(PageElementBuilder::new(page_element, size, node.page_num).build()),
        );
        view_root.page_num = node.page_num;

        if let Some(overflow) = node.overflow {
            view_root.overflow = overflow.iter().map(|o| o.page_num).collect::<Vec<_>>();

            for node in overflow {
                let page_element = PageLayout::Overflow(node.page);
                pmap.insert(
                    node.page_num,
                    Rc::new(PageElementBuilder::new(page_element, size, node.page_num).build()),
                );
            }
        }

        if let Some(children) = node.children {
            for child in children {
                let mut view_child = BTreeNodeView::default();
                Self::load_btree_node(child, pmap, &mut view_child, size);
                view_root.children.push(view_child);
            }
        }
    }

    fn load_freelist_pages(
        page: TrunkFreelistPage,
        page_num: usize,
        pages: &mut BTreeMap<usize, Rc<dyn PageView>>,
        reader: &Reader,
    ) -> Result<(), StdError> {
        let page_size = reader.db_header.page_size as usize;
        let page_element = PageLayout::TrunkFreelist(page.clone());
        pages.insert(
            page_num,
            Rc::new(PageElementBuilder::new(page_element, page_size, page_num).build()),
        );

        // Follow leaf pages from the trunk.
        if let Some(leaf_page_numbers) = page.leaf_page_numbers {
            for lpn in leaf_page_numbers {
                let lpn = lpn as usize;
                let leaf = reader.get_leaf_freelist_page(lpn)?;
                let page_element = PageLayout::LeafFreelist(leaf);
                pages.insert(
                    lpn,
                    Rc::new(PageElementBuilder::new(page_element, page_size, lpn).build()),
                );
            }
        };

        // Follow further trunk pages.
        match page.next_page {
            0 => Ok(()),
            page_num => {
                let page_num = page_num as usize;
                let next_page = reader.get_trunk_freelist_page(page_num)?;
                Self::load_freelist_pages(next_page, page_num, pages, reader)
            }
        }
    }
}
