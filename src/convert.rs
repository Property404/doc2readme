use crate::anchor_handler::AnchorHandlerFactory;
use crate::code_handler::CodeHandlerFactory;
use crate::header_handler::HeaderHandlerFactory;
use anyhow::{anyhow, Result};
use html2md::TagHandlerFactory;
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

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
            base_url: options
                .base_url
                .map(|mut s| {
                    // User has no business files as part of the base url, so assume they meant a
                    // directory
                    if !s.ends_with('/') {
                        s.push('/')
                    };

                    Url::parse(&s)
                })
                .transpose()?,
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

    #[test]
    fn strip_relative_links() {
        let markdown = html_to_readme(
            "<div class='docblock'><a href='fun'>hi</a></div>",
            Default::default(),
        )
        .unwrap();
        assert_eq!("hi", markdown.trim());

        let markdown = html_to_readme(
            "<div class='docblock'><a href='/fun'>hi</a></div>",
            Default::default(),
        )
        .unwrap();
        assert_eq!("hi", markdown.trim());

        let markdown = html_to_readme(
            "<div class='docblock'><a href='https://dagans.dev/fun'>hi</a></div>",
            Default::default(),
        )
        .unwrap();
        assert_eq!("[hi](https://dagans.dev/fun)", markdown.trim());
    }

    #[test]
    fn relative_links() {
        let options = Options {
            base_url: Some(String::from("https://dagans.dev/page/")),
        };

        let markdown = html_to_readme(
            "<div class='docblock'><a href='fun'>hi</a></div>",
            options.clone(),
        )
        .unwrap();
        assert_eq!("[hi](https://dagans.dev/page/fun)", markdown.trim());

        let markdown = html_to_readme(
            "<div class='docblock'><a href='/fun'>hi</a></div>",
            options.clone(),
        )
        .unwrap();
        assert_eq!("[hi](https://dagans.dev/fun)", markdown.trim());

        let markdown = html_to_readme(
            "<div class='docblock'><a href='https://dagans.dev/fun'>hi</a></div>",
            options.clone(),
        )
        .unwrap();
        assert_eq!("[hi](https://dagans.dev/fun)", markdown.trim());
    }
}
