mod anchor_handler;
mod header_handler;

use anchor_handler::AnchorHandlerFactory;
use anyhow::{anyhow, Result};
use header_handler::HeaderHandlerFactory;
use html2md::TagHandlerFactory;
use schmargs::{ArgsWithHelp, Schmargs};
use scraper::{Html, Selector};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str,
};

#[derive(Debug, Schmargs)]
#[schmargs(iterates_over=String)]
/// Construct README from rust docs
struct BareArgs {
    /// Base URL for relative links
    #[arg(short, long)]
    base_url: Option<String>,
    /// Output path
    #[arg(short, long)]
    output: Option<String>,
    /// Path to the file
    path: PathBuf,
}
type Args = ArgsWithHelp<BareArgs>;

fn main() -> Result<()> {
    let args = match Args::parse_env() {
        help @ Args::Help => {
            println!("{help}");
            return Ok(());
        }
        Args::Args(args) => args,
    };

    let html = fs::read_to_string(args.path)?;

    let html = Html::parse_fragment(&html);
    let docblock = html
        .select(&query(".docblock")?)
        .next()
        .unwrap()
        .inner_html();

    let mut handlers = HashMap::<String, Box<dyn TagHandlerFactory>>::new();
    handlers.insert(
        String::from("a"),
        Box::new(AnchorHandlerFactory {
            base_url: args.base_url,
        }),
    );
    handlers.insert(String::from("h1"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h2"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h3"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h4"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h5"), Box::new(HeaderHandlerFactory));
    handlers.insert(String::from("h6"), Box::new(HeaderHandlerFactory));

    let markdown = html2md::parse_html_custom(&docblock, &handlers);

    if let Some(output_file) = args.output {
        let mut file = File::create(output_file)?;
        file.write_all(markdown.as_bytes())?;
        file.write_all(b"\n")?;
    } else {
        println!("{}", markdown);
    }

    Ok(())
}

fn query(selector: impl AsRef<str>) -> Result<Selector> {
    Selector::parse(selector.as_ref()).map_err(|e| anyhow!("Failed to parse selector: {e}"))
}
