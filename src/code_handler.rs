use html2md::{StructuredPrinter, TagHandler, TagHandlerFactory};

use markup5ever_rcdom::{Handle, NodeData};

#[derive(Default)]
pub struct CodeHandler {
    code_type: Option<String>,
    language: Option<String>,
}

impl CodeHandler {
    // Used in both starting and finishing handling
    fn do_handle(&mut self, printer: &mut StructuredPrinter, start: bool) {
        let immediate_parent = printer.parent_chain.last().unwrap().to_owned();
        if self.code_type.as_ref().expect("Expected `code_type` set") == "code"
            && immediate_parent == "pre"
        {
            // we are already in "code" mode
            return;
        }

        match self
            .code_type
            .as_ref()
            .expect("Bug: `code_type` not set")
            .as_ref()
        {
            "pre" => {
                // code block should have its own paragraph
                if !printer.data.ends_with('\n') {
                    printer.insert_newline();
                }
                if start {
                    printer.insert_newline();
                }
                printer.append_str("```");
                if start {
                    if let Some(language) = &self.language {
                        printer.append_str(language.as_ref());
                    }
                } else {
                    printer.insert_newline();
                }
                printer.insert_newline();
            }
            "code" | "samp" => printer.append_str("`"),
            _ => {}
        }
    }
}

impl TagHandler for CodeHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        if let NodeData::Element {
            ref name,
            ref attrs,
            ..
        } = tag.data
        {
            self.code_type = Some(name.local.to_string());
            let classes = attrs
                .borrow()
                .iter()
                .find(|attr| attr.name.local.to_string() == "class")
                .cloned();
            if let Some(classes) = classes {
                let classes = classes.value.split_whitespace();
                for class in classes {
                    if class == "rust" {
                        self.language = Some(String::from("rust"))
                    } else if let Some(language) = class.strip_prefix("language-") {
                        self.language = Some(language.into())
                    }
                }
            }
        }

        self.do_handle(printer, true);
    }
    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        self.do_handle(printer, false);
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CodeHandlerFactory;

impl TagHandlerFactory for CodeHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::<CodeHandler>::default()
    }
}
