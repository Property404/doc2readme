mod ahandler;

use ahandler::AnchorHandlerFactory;
use anyhow::{anyhow, Result};
use html2md::TagHandlerFactory;
use schmargs::{ArgsWithHelp, Schmargs};
use scraper::{Html, Selector};
use std::{collections::HashMap, fs, path::PathBuf, str};

#[derive(Debug, Schmargs)]
#[schmargs(iterates_over=String)]
/// Construct README from rust docs
struct BareArgs {
    /// Base URL for relative links
    #[arg(short, long)]
    base_url: Option<String>,
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

    println!("{}", html2md::parse_html_custom(&docblock, &handlers));

    Ok(())
}

fn query(selector: impl AsRef<str>) -> Result<Selector> {
    Selector::parse(selector.as_ref()).map_err(|e| anyhow!("Failed to parse selector: {e}"))
}
