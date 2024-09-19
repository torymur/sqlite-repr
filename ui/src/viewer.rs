//! Database UI Viewer.

use std::collections::BTreeMap;
use std::rc::Rc;

use parser::{Cell, OverflowPage, Reader, StdError, TrunkFreelistPage};

use crate::included_db::INCLUDED_DB;
use crate::{PageElementBuilder, PageLayout, PageView};

#[derive(Debug)]
pub struct Viewer {
    pub included_db: BTreeMap<&'static str, (&'static [u8], &'static [&'static str])>,
    pub pages: Vec<Rc<dyn PageView>>,
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

        for n in 1..reader.pages_total() + 1 {
            if pages_map.contains_key(&n) {
                // It was already filled.
                continue;
            };

            let page = match reader.get_btree_page(n) {
                Ok(page) => page,
                Err(_) => continue,
            };
            // Check for overflow information in each cell of the page.
            for cell in &page.cells {
                let cell_overflow = match cell {
                    Cell::TableInterior(_) => continue, // the only one without overflow
                    Cell::TableLeaf(c) => &c.overflow,
                    Cell::IndexLeaf(c) => &c.overflow,
                    Cell::IndexInterior(c) => &c.overflow,
                };
                match cell_overflow {
                    None => continue,
                    Some(overflow) => {
                        let opage = reader
                            .get_overflow_page(overflow.units.to_vec(), overflow.page as usize)?;
                        Self::load_overflow_page(
                            opage,
                            overflow.page as usize,
                            &mut pages_map,
                            &reader,
                        )?;
                    }
                };
            }
            let page_element = PageLayout::Btree(page);
            pages_map.insert(
                n,
                Rc::new(PageElementBuilder::new(page_element, size, n).build()),
            );
        }

        let pages: Vec<Rc<dyn PageView>> = pages_map.into_values().collect();
        Ok(Self { included_db, pages })
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

    fn load_overflow_page(
        page: OverflowPage,
        page_num: usize,
        pages: &mut BTreeMap<usize, Rc<dyn PageView>>,
        reader: &Reader,
    ) -> Result<(), StdError> {
        let page_size = reader.db_header.page_size as usize;
        let page_element = PageLayout::Overflow(page.clone());
        pages.insert(
            page_num,
            Rc::new(PageElementBuilder::new(page_element, page_size, page_num).build()),
        );
        // Follow further overflow pages.
        match page.next_page {
            0 => Ok(()),
            page_num => {
                let page_num = page_num as usize;
                let next_page = reader.get_overflow_page(page.overflow_units, page_num)?;
                Self::load_overflow_page(next_page, page_num, pages, reader)
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
