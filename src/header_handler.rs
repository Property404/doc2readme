use html2md::{StructuredPrinter, TagHandler, TagHandlerFactory};
use markup5ever_rcdom::{Handle, NodeData};

#[derive(Default)]
pub struct HeaderHandler {
    header_type: String,
}

impl TagHandler for HeaderHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.header_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new(),
        };

        printer.insert_newline();
        printer.insert_newline();
        match self.header_type.as_ref() {
            "h1" => printer.append_str("# "),
            "h2" => printer.append_str("## "),
            "h3" => printer.append_str("### "),
            "h4" => printer.append_str("#### "),
            "h5" => printer.append_str("##### "),
            "h6" => printer.append_str("###### "),
            _ => {
                panic!("This is not a header")
            }
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HeaderHandlerFactory;

impl TagHandlerFactory for HeaderHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(HeaderHandler {
            header_type: Default::default(),
        })
    }
}
