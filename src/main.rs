#![allow(unused_imports)]
use anyhow::Result;
use schmargs::{ArgsWithHelp, Schmargs};
use std::{fs, path::PathBuf};
use tl::{self, ParserOptions};

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

    let html = tl::parse(&html, ParserOptions::default())?;
    let parser = html.parser();
    let main_content = html
        .get_element_by_id("main-content")
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap();

        println!("...");
    for docblock in main_content.query_selector(parser, ".docblock").unwrap() {
        println!("Wow!");
        let docblock = docblock.get(parser).unwrap().as_tag().unwrap();
        let mut markdown = String::new();
        for child in docblock.children().all(parser) {
            println!("{child:?}");
        }
    }

    Ok(())
}
