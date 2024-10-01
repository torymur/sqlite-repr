//! Database UI Viewer.

use std::collections::BTreeMap;
use std::rc::Rc;

use parser::*;

use crate::included_db::INCLUDED_DB;
use crate::{BTreeNodeView, BTreeView, PageElementBuilder, PageLayout, PageView};

#[derive(Debug)]
pub struct Viewer {
    pub included_db: BTreeMap<&'static str, (&'static [u8], &'static [&'static str])>,
    pub pages: Vec<Rc<dyn PageView>>,
    pub btrees: Vec<BTreeView>,
}

pub type Result<T, E = StdError> = std::result::Result<T, E>;

impl Viewer {
    pub fn new_from_included(name: &str) -> Result<Self, StdError> {
        let included_db: BTreeMap<&'static str, (&'static [u8], &'static [&'static str])> =
            BTreeMap::from_iter(INCLUDED_DB.iter().copied());
        let (bytes, _) = included_db.get(name).ok_or("This db is not included.")?;
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

        Ok(Self {
            included_db,
            pages,
            btrees: view_trees,
        })
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
