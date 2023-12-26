use crate::anchor_handler::AnchorHandlerFactory;
use crate::header_handler::HeaderHandlerFactory;
use anyhow::{anyhow, Result};
use html2md::TagHandlerFactory;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Options {
    pub base_url: Option<String>,
}

pub fn html_to_readme(html: &str, options: Options) -> Result<String> {
    let html = Html::parse_fragment(html);
    let docblock = html
        .select(&query(".docblock")?)
        .next()
        .ok_or_else(|| anyhow!("Could not find .docblock element. Is this crate documented?"))?
        .inner_html();

    let mut handlers = HashMap::<String, Box<dyn TagHandlerFactory>>::new();
    handlers.insert(
        String::from("a"),
        Box::new(AnchorHandlerFactory {
            base_url: options.base_url,
        }),
    );
    handlers.insert(String::from("h1"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h2"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h3"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h4"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h5"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h6"), Box::new(HeaderHandlerFactory));

    let mut markdown = html2md::parse_html_custom(&docblock, &handlers);

    // minjinja strips newlines, which is only sometimes what we want
    if !markdown.ends_with('\n') {
        markdown.push('\n');
    }

    Ok(markdown)
}

fn query(selector: impl AsRef<str>) -> Result<Selector> {
    Selector::parse(selector.as_ref()).map_err(|e| anyhow!("Failed to parse selector: {e}"))
}
