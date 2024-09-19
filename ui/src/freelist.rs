use std::rc::Rc;

use crate::{Field, Part, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct NextPagePart {
    fields: Vec<Rc<Field>>,
}

impl NextPagePart {
    pub fn new(next_page: u32) -> Self {
        let fields = vec![Rc::new(Field::new(
            "Value is the next freelist trunk page in a linked list.",
            0,
            4,
            Value::PageNumber(next_page),
            "",
        ))];
        Self { fields }
    }
}

impl Part for NextPagePart {
    fn label(&self) -> String {
        "Next freelist trunk page".to_string()
    }

    fn desc(&self) -> &'static str {
        "Freelist trunk pages form a linked list. The first four bytes of each freelist trunk page are a big-endian integer which is the page number of the next page in the chain, or zero for the final page in the chain."
    }

    fn color(&self) -> String {
        "green".to_string()
    }

    fn fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeafPageAmountPart {
    fields: Vec<Rc<Field>>,
}

impl LeafPageAmountPart {
    pub fn new(leaf_page_amount: u32) -> Self {
        let fields = vec![Rc::new(Field::new(
            "",
            4,
            4,
            Value::U32(leaf_page_amount),
            "",
        ))];
        Self { fields }
    }
}

impl Part for LeafPageAmountPart {
    fn label(&self) -> String {
        "Amount of leaf pages".to_string()
    }

    fn desc(&self) -> &'static str {
        "Amount of freelist leaf pages to follow found on this trunk page."
    }

    fn color(&self) -> String {
        "orange".to_string()
    }

    fn fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeafPageNumbersPart {
    fields: Vec<Rc<Field>>,
}

impl LeafPageNumbersPart {
    pub fn new(page_numbers: &[u32]) -> Self {
        let mut offset = 8;
        let mut fields = vec![];

        for pn in page_numbers {
            fields.push(Rc::new(Field::new(
                "Freelist leaf page number.",
                offset,
                4,
                Value::PageNumber(*pn),
                "",
            )));
            offset += 4;
        }
        Self { fields }
    }
}

impl Part for LeafPageNumbersPart {
    fn label(&self) -> String {
        "Array of leaf page numbers".to_string()
    }

    fn desc(&self) -> &'static str {
        "A freelist trunk page consists of an array of 4-byte big-endian integers. The size of the array is as many integers as will fit in the usable space of a page. Call the second integer on a freelist trunk page L. If L >= 0 then integers with array indexes between 2 and L+1 inclusive contain page numbers for freelist leaf pages."
    }

    fn color(&self) -> String {
        "green".to_string()
    }

    fn fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnallocatedPart {
    fields: Vec<Rc<Field>>,
}

impl UnallocatedPart {
    pub fn new(unallocated: &[u8], offset: usize) -> Self {
        let fields = vec![Rc::new(Field::new(
            "",
            offset,
            unallocated.len(),
            Value::Unallocated(unallocated.into()),
            "",
        ))];
        Self { fields }
    }
}

impl Part for UnallocatedPart {
    fn label(&self) -> String {
        "Unallocated space".to_string()
    }

    fn desc(&self) -> &'static str {
        "The whole area of the freelist leaf is not allocated."
    }

    fn color(&self) -> String {
        "orange".to_string()
    }

    fn fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }
}
