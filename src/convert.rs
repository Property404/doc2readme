use crate::anchor_handler::AnchorHandlerFactory;
use crate::code_handler::CodeHandlerFactory;
use crate::header_handler::HeaderHandlerFactory;
use anyhow::{anyhow, Result};
use html2md::TagHandlerFactory;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
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
    handlers.insert(String::from("code"), Box::new(CodeHandlerFactory));
    handlers.insert(String::from("pre"), Box::new(CodeHandlerFactory));

    let markdown = html2md::parse_html_custom(&docblock, &handlers);

    Ok(markdown)
}

fn query(selector: impl AsRef<str>) -> Result<Selector> {
    Selector::parse(selector.as_ref()).map_err(|e| anyhow!("Failed to parse selector: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codeblock() {
        let markdown = html_to_readme("<div class=\"docblock\"><div class=\"example-wrap\"><pre class=\"language-notrust\"><code>hello you!
</code></pre></div></div>", Default::default()).unwrap();

        assert_eq!("```notrust\nhello you!\n```", markdown.trim());
    }
}
