#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    // Spans
    Text(String),
    CodeSpan(Vec<Item>),
    Link(String, Vec<Item>),

    // Blocks
    Header1(Vec<Item>),
    Header2(Vec<Item>),
    Header3(Vec<Item>),
    CodeBlock(Vec<Item>),
    Paragraph(Vec<Item>),
    UnorderedList(Vec<Item>),

    // Something else
    ListItem(Vec<Item>),
}

impl Item {
    const fn is_span(&self) -> bool {
        match Self {
            Self::Text(_) => true,
            Self::Link(_) => true,
            Self::CodeSpan(_) => true,
            _ => false,
        }
    }
    const fn is_block(&self) -> bool {
        !self.is_span()
    }
}
