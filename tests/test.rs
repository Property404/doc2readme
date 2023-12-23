use anyhow::Result;
use pretty_assertions::assert_eq;
use std::{env, fs, path::Path, process::Command};
use tempfile::TempDir;

#[test]
fn integration_test() -> Result<()> {
    let paths = fs::read_dir("assets/readmes/").unwrap();

    for path in paths {
        let path = path?;
        test_project(path.path())?;
    }

    Ok(())
}

fn test_project(markdown_file: impl AsRef<Path>) -> Result<()> {
    // Create cargo project
    let cargo_dir = TempDir::new()?;
    let copy_options = Default::default();
    fs_extra::dir::copy("assets/bare/", cargo_dir.path(), &copy_options)?;
    let cargo_dir = cargo_dir.path().join("bare");
    fs::copy(markdown_file.as_ref(), cargo_dir.join("README.md"))?;
    env::set_current_dir(cargo_dir)?;

    // Build documentation
    assert!(Command::new("cargo").arg("doc").output()?.status.success());

    // Extract docs
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("target/doc/bare/index.html")
        .arg("-o")
        .arg("EXTRACTED.md")
        .ok()?;

    let original_text = fs::read_to_string("README.md")?;
    let extracted_text = fs::read_to_string("EXTRACTED.md")?;

    assert_eq!(original_text, extracted_text);
    Ok(())
}
