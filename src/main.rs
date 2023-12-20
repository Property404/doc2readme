#![allow(unused_imports)]
mod markdown;
use anyhow::{anyhow, Result};
use ego_tree::NodeRef;
use markdown::Item;
use schmargs::{ArgsWithHelp, Schmargs};
use scraper::{Html, Node, Selector};
use std::{
    collections::VecDeque,
    fmt::{self, Write},
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

    println!("...");
    let mut markdown = Vec::new();
    for docblock in main_content.select(&query(".docblock")?) {
        println!("Wow!");
        for child in docblock.children() {
            markdown.extend(handle_node(&child)?);
            //println!("CHILD: {child:?}");
        }
    }
    println!("Markdown: {markdown:?}");
    let mut string = String::new();
    Item::print_all(&mut string, &markdown)?;
    println!("{string}");

    Ok(())
}

fn handle_children<'a>(children: impl Iterator<Item = NodeRef<'a, Node>>) -> Result<Vec<Item>> {
    let mut vec = Vec::new();
    for child in children {
        vec.extend(handle_node(&child)?);
    }
    Ok(vec)
}

#[inline(never)]
fn handle_node(node: &NodeRef<Node>) -> Result<Vec<Item>> {
    match node.value() {
        Node::Text(text) => {
            return Ok(vec![Item::Text(format!(
                "{}",
                str::from_utf8(text.as_bytes())?.replace("\n"," ").replace("  ","")
            ))]);
        }
        Node::Element(element) => {
            let name = element.name();
            if name == "h2" {
                return Ok(vec![Item::Header2(handle_children(node.children())?)]);
            } else if name == "h3" {
                return Ok(vec![Item::Header3(handle_children(node.children())?)]);
            } else if name == "code" {
                return Ok(vec![Item::CodeSpan(handle_children(node.children())?)]);
            } else if name == "p" {
                return Ok(vec![Item::Paragraph(handle_children(node.children())?)]);
            } else if name == "ul" {
                return Ok(vec![Item::UnorderedList(handle_children(node.children())?)]);
            } else if name == "li" {
                return Ok(vec![Item::ListItem(handle_children(node.children())?)]);
            } else if name == "a" {
                return Ok(vec![Item::Link(
                    "url".into(),
                    handle_children(node.children())?,
                )]);
            } else {
                println!("<{name}/>");
            }
        }
        _ => {}
    }
    Ok(Vec::default())
}

fn query(selector: impl AsRef<str>) -> Result<Selector> {
    Selector::parse(selector.as_ref()).map_err(|e| anyhow!("Failed to parse selector: {e}"))
}
