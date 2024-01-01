//! Convert crate documention into a README
//!
//! Alternative to [cargo-readme](https://docs.rs/cargo-readme). Unlike `cargo-readme`,
//! `cargo-doc2readme` parses the output of rustdoc instead of extracting the doc comments directly
//! from the rust source. The main advantage here is that `cargo-doc2readme` can handle relative
//! links in crate documentation.
//!
//! # Basic Usage
//!
//! Install:
//!
//! ```shell
//! cargo install cargo-doc2readme --git https://github.com/Property404/doc2readme
//! ```
//!
//! Usage:
//!
//! ```shell
//! $ cargo doc2readme -o README.md
//! ```
//!
//! # Templating
//!
//! `cargo-doc2readme` uses [minjinja](https://docs.rs/minijinja) as its
//! templating engine, which happens to be a superset of `cargo-readme`'s templating engine. Like
//! `cargo-readme`, `cargo-doc2readme` uses `README.tpl` as the template by default if it exists,
//! but this can be overridden with the `--template` command line option.
//!
//! The default template is:
//!
//! ```jinja
#![doc = include_str!("./DEFAULT_TEMPLATE.tpl")]
//! ```
//!
//! ## Template variables
//!
//! * `crate` - the crate name, alias for `package.name`
//! * `license` - the crate license, alias for `package.license`
//! * `readme` - the generated readme text
//! * `version` - the crate version, alias for `package.version`
//! * `package` - All package keys
//!
//! # Todo
//!
//! * Get dependencies published
mod anchor_handler;
mod code_handler;
mod convert;
mod header_handler;
mod manifest;

use anyhow::{anyhow, bail, Result};
use convert::Options;
use manifest::ProjectInfo;
use minijinja::{context, Environment};
use schmargs::{ArgsWithHelp, Schmargs};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    process::{self, Command},
    str,
};

const DEFAULT_TEMPLATE_PATH: &str = "README.tpl";

#[derive(Debug, Schmargs)]
#[schmargs(iterates_over=String)]
/// Construct README from rust docs
struct BareArgs {
    /// Unpin the version for `std`/`core`/`alloc` docs links
    #[arg(long)]
    unpin_std_docs: bool,
    /// Don't use any templating
    #[arg(long)]
    no_template: bool,
    /// Arguments to pass to `cargo doc`
    #[arg(long, default_value)]
    rustdoc_args: Vec<String>,
    /// Base URL for relative links
    #[arg(short = 'u', long)]
    base_url: Option<String>,
    /// The template to use, if any
    #[arg(short, long)]
    template: Option<String>,
    /// Output path
    #[arg(short, long)]
    output: Option<String>,
    /// The crate from which to extract docs
    #[arg(value_name = "CRATE")]
    crate_name: Option<String>,
}
type Args = ArgsWithHelp<BareArgs>;

fn main() -> Result<()> {
    // We have to skip twice because `cargo doc2readme` invokes as `cargo-doc2readme doc2readme`
    //
    // Allow "readme" so we can be a drop-in replacement for `cargo-readme`
    let args = env::args().skip(1).enumerate().filter_map(|(i, val)| {
        if i == 0 && (val == "doc2readme" || val == "readme") {
            None
        } else {
            Some(val)
        }
    });

    let args = match Args::parse(args) {
        Ok(help @ Args::Help) => {
            println!("{help}");
            return Ok(());
        }
        Ok(Args::Args(args)) => args,
        Err(err) => {
            eprintln!("Error: {err}");
            eprintln!("Usage: {}", Args::USAGE);
            process::exit(1);
        }
    };

    // Run `cargo doc` so docs and `target` directory is created
    Command::new("cargo")
        .arg("doc")
        .arg("--no-deps")
        .args(args.rustdoc_args.into_iter())
        .status()?;

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

    if doc_path.metadata().is_err() {
        bail!("Cannot find '{}'", doc_path.display());
    }

    let html = fs::read_to_string(doc_path)?;
    let mut markdown = convert::html_to_readme(
        &html,
        Options {
            base_url: args.base_url,
            unpin_std_docs: args.unpin_std_docs,
        },
    )?;

    // Template markdown
    if !args.no_template {
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
        markdown = template.render(context!(
                crate => crate_name,
                readme => markdown,
                version => manifest.package.as_ref().map(|p|p.version.clone()),
                license => manifest.package.as_ref().map(|p|p.license.clone()),
                package => manifest.package.clone(),
        ))?;
    }

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
