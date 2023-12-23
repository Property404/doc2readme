use anyhow::Result;
use pretty_assertions::assert_eq;
use std::{env, fs, path::Path, process::Command};
use tempfile::TempDir;

#[test]
fn integration_test() -> Result<()> {
    let paths = fs::read_dir("assets/").unwrap();

    for project in paths {
        let project = project?;
        test_project(project.path())?;
    }

    Ok(())
}

fn test_project(project_path: impl AsRef<Path>) -> Result<()> {
    // Create cargo project
    let cargo_dir = TempDir::new()?;
    let copy_options = Default::default();
    fs_extra::dir::copy(project_path.as_ref(), cargo_dir.path(), &copy_options)?;
    let cargo_dir = cargo_dir
        .path()
        .join(project_path.as_ref().file_name().unwrap());
    env::set_current_dir(cargo_dir)?;

    // Build documentation
    assert!(Command::new("cargo").arg("doc").output()?.status.success());

    // Extract docs
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("-o")
        .arg("EXTRACTED.md")
        .ok()?;

    let original_text = fs::read_to_string("README.md")?;
    let extracted_text = fs::read_to_string("EXTRACTED.md")?;

    assert_eq!(original_text, extracted_text);
    Ok(())
}
