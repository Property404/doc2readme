use html2md::{StructuredPrinter, TagHandler, TagHandlerFactory};
use markup5ever_rcdom::{Handle, NodeData};
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

static STD_DOCS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https://doc.rust-lang.org/([0-9]\.[0-9]{1,5}\.[0-9]{1,5}|stable|beta|nightly)/(core|alloc|std)/(.*)$").unwrap()
});

#[derive(Clone, Debug, Default)]
pub(crate) struct AnchorHandler {
    base_url: Option<Url>,
    start_pos: usize,
    // This is not necessarily a full URL - can be an anchor, for example
    url: Option<String>,
    skip_descendents: bool,
    unpin_std_docs: bool,
}

impl TagHandler for AnchorHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.data.len();

        // try to extract a hyperlink
        let mut url = match tag.data {
            NodeData::Element { ref attrs, .. } => {
                let attrs = attrs.borrow();

                // Ignore tooltips
                if let Some(class) = attrs
                    .iter()
                    .find(|attr| attr.name.local.to_string() == "class")
                {
                    if class.value == "tooltip".into() {
                        self.skip_descendents = true;
                        return;
                    }
                }

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

        // Don't convert header links
        let immediate_parent = printer.parent_chain.last().unwrap().to_owned();
        if ["h1", "h2", "h3", "h4", "h5", "h6"].contains(&immediate_parent.as_str())
            && url.starts_with('#')
        {
            return;
        }

        if self.unpin_std_docs {
            if let Some(captures) = STD_DOCS_REGEX.captures(&url) {
                let captures: [&str; 3] = captures.extract().1;
                url = format!("https://doc.rust-lang.org/{}/{}", captures[1], captures[2]);
            }
        }

        if url.starts_with("https://") || url.starts_with("http://") || url.starts_with('#') {
            self.url = Some(url);
            return;
        }

        self.url = self
            .base_url
            .as_ref()
            .map(|base_url| base_url.join(&url).map(|url| url.to_string()))
            .transpose()
            .unwrap_or_else(|err| {
                eprintln!("Error parsing URL: {err}");
                None
            })
    }

    fn skip_descendants(&self) -> bool {
        self.skip_descendents
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
    pub base_url: Option<Url>,
    pub unpin_std_docs: bool,
}

impl TagHandlerFactory for AnchorHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(AnchorHandler {
            base_url: self.base_url.clone(),
            unpin_std_docs: self.unpin_std_docs,
            ..Default::default()
        })
    }
}
