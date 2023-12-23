//! Convert crate documention into a README
//!
//! Alternative to [cargo-readme](https://docs.rs/cargo-readme). Unlike `cargo-readme`,
//! `cargo-doc2readme` parses the output of rustdoc instead of extracting the doc comments directly
//! from the rust source.
//!
//! # Basic Usage
//!
//! Install:
//!
//! ```notrust
//! cargo install --git https://github.com/Property404/doc2readme
//! ```
//!
//! Usage:
//!
//! ```notrust
//! $ cargo doc2readme -o README.md
//! ```
//!
//! # Templating
//!
//! `cargo-doc2readme` usages [minjinja](https://docs.rs/minijinja) as its
//! templating engine, which happens to be a superset of `cargo-readme`'s templating engine. Like
//! `cargo-readme`, `cargo-doc2readme` uses `README.tpl` as the template by default if it exists.
//!
//! The default template is:
//!
//! ```notrust
#![doc = include_str!("./DEFAULT_TEMPLATE.tpl")]
//! ```
//!
//! ## Template variables
//!
//! * crate - the crate name
//! * license - the crate license
//! * readme - the generated readme text
//! * version - the crate version
mod anchor_handler;
mod header_handler;
mod manifest;

use anchor_handler::AnchorHandlerFactory;
use anyhow::{anyhow, bail, Result};
use header_handler::HeaderHandlerFactory;
use html2md::TagHandlerFactory;
use manifest::ProjectInfo;
use minijinja::{context, Environment};
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

const DEFAULT_TEMPLATE_PATH: &str = "README.tpl";

#[derive(Debug, Schmargs)]
#[schmargs(iterates_over=String)]
/// Construct README from rust docs
struct BareArgs {
    /// The template to use, if any
    #[arg(short, long)]
    template: Option<String>,
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

    let mut templates = Environment::new();

    let template = if let Some(template_path) = args.template {
        fs::read_to_string(template_path)?
    } else if Path::new(DEFAULT_TEMPLATE_PATH).is_file() {
        fs::read_to_string(DEFAULT_TEMPLATE_PATH)?
    } else {
        include_str!("DEFAULT_TEMPLATE.tpl").into()
    };
    templates.add_template("template", &template)?;
    let template = templates.get_template("template")?;

    let markdown = html2md::parse_html_custom(&docblock, &handlers);
    let mut markdown = template.render(context!(
    crate => crate_name,
    readme => markdown,
    version => manifest.package.as_ref().map(|p|p.version.clone()),
    license => manifest.package.as_ref().map(|p|p.license.clone())
    ))?;
    // minjinja strips newlines, which is only sometimes what we want
    if !markdown.ends_with('\n') {
        markdown.push('\n');
    }

    if let Some(output_file) = args.output {
        let mut file = File::create(output_file)?;
        file.write_all(markdown.as_bytes())?;
    } else {
        print!("{}", markdown);
    }

    Ok(())
}

fn query(selector: impl AsRef<str>) -> Result<Selector> {
    Selector::parse(selector.as_ref()).map_err(|e| anyhow!("Failed to parse selector: {e}"))
}
