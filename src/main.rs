use anyhow::{anyhow, Result};
use schmargs::{ArgsWithHelp, Schmargs};
use scraper::{Html, Selector};
use std::{
    fs,
    path::PathBuf,
    str,
};

#[derive(Debug, Schmargs)]
#[schmargs(iterates_over=String)]
/// Construct README from rust docs
struct BareArgs {
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

    println!("Path: {}", args.path.display());

    let html = fs::read_to_string(args.path)?;

    let html = Html::parse_fragment(&html);
    let main_content = html.select(&query("#main-content")?).next().unwrap();
    let docblock = main_content
        .select(&query(".docblock")?)
        .next()
        .unwrap()
        .inner_html();

    println!("{}", html2md::parse_html(&docblock));

    Ok(())
}

fn query(selector: impl AsRef<str>) -> Result<Selector> {
    Selector::parse(selector.as_ref()).map_err(|e| anyhow!("Failed to parse selector: {e}"))
}
