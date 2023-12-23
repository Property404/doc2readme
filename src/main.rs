//! Convert crate documention into a README
//!
//! Usage
//!
//! ```ignore
//! bla bla bla
//! ```
//!
//! # Features
//!
//! `std` - loljk this isn't a feature
mod anchor_handler;
mod header_handler;
mod manifest;

use anchor_handler::AnchorHandlerFactory;
use anyhow::{anyhow, bail, Result};
use header_handler::HeaderHandlerFactory;
use html2md::TagHandlerFactory;
use manifest::ProjectInfo;
use schmargs::{ArgsWithHelp, Schmargs};
use scraper::{Html, Selector};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
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
    /// The crate from which to extract docs
    crate_name: Option<String>,
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

    let project_info = ProjectInfo::new()?;
    let (crate_name, manifest) = if let Some(crate_name) = args.crate_name {
        (
            crate_name.clone(),
            project_info
                .manifests
                .get(&crate_name)
                .ok_or_else(|| anyhow!("No such crate `{crate_name}`"))?
                .clone(),
        )
    } else {
        let manifests = project_info.manifests;
        if manifests.len() > 1 {
            eprintln!("Multiple crates found:");
            for (krate, _) in manifests {
                eprintln!("\t{krate}");
            }
            bail!("Could not select a crate");
        }
        manifests
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No crates found"))?
    };

    let doc_path = Path::new(".")
        .join(project_info.target_dir)
        .join(format!("doc/{}/index.html", crate_name.replace('-', "_")));

    Command::new("cargo").arg("doc").status()?;
    if doc_path.metadata().is_err() {
        bail!("Cannot find '{}'", doc_path.display());
    }

    let html = fs::read_to_string(doc_path)?;

    let html = Html::parse_fragment(&html);
    let docblock = html
        .select(&query(".docblock")?)
        .next()
        .ok_or_else(|| anyhow!("Could not find .docblock element. Is this crate documented?"))?
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
