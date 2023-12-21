use html2md::{StructuredPrinter, TagHandler, TagHandlerFactory};

use markup5ever_rcdom::{Handle, NodeData};

#[derive(Clone, Debug, Default)]
pub(crate) struct AnchorHandler {
    base_url: Option<String>,
    start_pos: usize,
    url: Option<String>,
}

impl TagHandler for AnchorHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.data.len();

        // try to extract a hyperlink
        let url = match tag.data {
            NodeData::Element { ref attrs, .. } => {
                let attrs = attrs.borrow();
                let href = attrs
                    .iter()
                    .find(|attr| attr.name.local.to_string() == "href");
                match href {
                    Some(link) => link.value.to_string(),
                    None => String::new(),
                }
            }
            _ => String::new(),
        };

        if url.starts_with("https://") || url.starts_with('#') {
            self.url = Some(url);
            return;
        }

        self.url = self
            .base_url
            .as_ref()
            .map(|base_url| base_url.to_string() + &url)
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // add braces around already present text, put an url afterwards
        if let Some(url) = &self.url {
            printer.insert_str(self.start_pos, "[");
            printer.append_str(&format!("]({})", url))
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct AnchorHandlerFactory {
    pub base_url: Option<String>,
}

impl TagHandlerFactory for AnchorHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(AnchorHandler {
            base_url: self.base_url.clone(),
            ..Default::default()
        })
    }
}
