use std::rc::Rc;

use parser::*;

use crate::{Field, PageView, Part, Value};

#[derive(Debug, Clone)]
pub struct OverflowPageElement {
    pub id: usize,
    pub page: Rc<OverflowPage>,
    pub size: usize,
    parts: Vec<Rc<dyn Part>>,
}

pub struct OverflowPageElementBuilder {
    pub id: usize,
    pub page: Rc<OverflowPage>,
    pub size: usize,
    #[allow(dead_code)]
    parts: Option<Vec<Rc<dyn Part>>>,
}

impl OverflowPageElementBuilder {
    pub fn new(page: OverflowPage, size: usize, page_num: usize) -> Self {
        Self {
            id: page_num,
            page: Rc::new(page),
            size,
            parts: None,
        }
    }

    pub fn build(self) -> OverflowPageElement {
        let parts = self.build_parts();
        OverflowPageElement {
            id: self.id,
            page: self.page,
            size: self.size,
            parts,
        }
    }

    fn build_parts(&self) -> Vec<Rc<dyn Part>> {
        let mut parts: Vec<Rc<dyn Part>> = vec![
            Rc::new(NextPagePart::new(self.page.next_page)),
            Rc::new(DataPart::new(&self.page.data)),
        ];

        if let Some(unallocated) = &self.page.unallocated {
            parts.push(Rc::new(UnallocatedOverflowPart::new(
                unallocated,
                &self.page,
            )));
        }
        parts
    }
}

impl PageView for OverflowPageElement {
    fn id(&self) -> usize {
        self.id
    }

    fn size(&self) -> usize {
        self.size
    }

    fn label(&self) -> String {
        "ᨒ  Overflow".to_string()
    }

    fn desc(&self) -> &'static str {
        "When the size of payload for a cell exceeds a certain threshold, then only the first few bytes of the payload are stored on the b-tree page and the balance is stored in a linked list of content overflow pages."
    }

    fn parts(&self) -> &[Rc<dyn Part>] {
        self.parts.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NextPagePart {
    fields: Vec<Rc<Field>>,
}

impl NextPagePart {
    pub fn new(next_page: u32) -> Self {
        let fields = vec![Rc::new(Field::new(
            "Value is the next overflow page in a linked list.",
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
        "Next overflow page".to_string()
    }

    fn desc(&self) -> &'static str {
        "Overflow pages form a linked list. The first four bytes of each overflow page are a big-endian integer which is the page number of the next page in the chain, or zero for the final page in the chain."
    }

    fn color(&self) -> String {
        "green".to_string()
    }

    fn fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataPart {
    fields: Vec<Rc<Field>>,
}

impl DataPart {
    pub fn new(overflow: &[OverflowData]) -> Self {
        let mut offset = 4;
        let mut fields = vec![];

        for data in overflow {
            let style = if data.bytes.is_empty() {
                "pattern-vertical-lines pattern-white pattern-bg-slate-200 pattern-size-1 pattern-opacity-60 bg-slate-390"
            } else {
                "bg-slate-390"
            };
            fields.push(Rc::new(Field::new(
                "Cell's payload spilled over.",
                offset,
                data.bytes.len(),
                Value::Record(data.value.clone()),
                style,
            )));
            offset += data.bytes.len();
        }
        Self { fields }
    }
}

impl Part for DataPart {
    fn label(&self) -> String {
        "Cell's payload overflow".to_string()
    }

    fn desc(&self) -> &'static str {
        "The amount of payload that spills onto overflow pages also depends on the page type. The overflow thresholds are designed to give a minimum fanout of 4 for index b-trees and to make sure enough of the payload is on the b-tree page that the record header can usually be accessed without consulting an overflow page."
    }

    fn color(&self) -> String {
        "orange".to_string()
    }

    fn fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnallocatedOverflowPart {
    fields: Vec<Rc<Field>>,
}

impl UnallocatedOverflowPart {
    pub fn new(unallocated: &[u8], page: &OverflowPage) -> Self {
        let mut offset = 4;
        page.data.iter().for_each(|d| offset += d.bytes.len());
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

impl Part for UnallocatedOverflowPart {
    fn label(&self) -> String {
        "Unallocated space".to_string()
    }

    fn desc(&self) -> &'static str {
        "The area in between the last cell payload and end of the overflow page."
    }

    fn color(&self) -> String {
        "green".to_string()
    }

    fn fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }
}
