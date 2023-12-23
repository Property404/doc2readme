use anyhow::{bail, Result};
use cargo_toml::Manifest;
use std::{collections::HashMap, env, path::PathBuf};

#[derive(Clone, Debug)]
pub struct ProjectInfo {
    pub target_dir: PathBuf,
    pub manifests: HashMap<String, Manifest>,
}

impl ProjectInfo {
    pub fn new() -> Result<Self> {
        let mut target_dir = None;
        let mut manifests = None;

        let mut current_path = env::current_dir()?;
        while target_dir.is_none() && manifests.is_none() {
            println!("{current_path:?}");
            if current_path.parent().expect("No parent") == current_path {
                if manifests.is_none() {
                    bail!("Not in a Cargo project directory");
                }
            }

            let manifest_path = current_path.join("Cargo.toml");
            if manifests.is_none() && manifest_path.is_file() {
                let manifest = Manifest::from_path(manifest_path)?;
                if manifest.workspace.is_some() {
                    todo!("Can't handle workspaces yet");
                }

                let Some(ref package) = manifest.package else {
                    bail!("Manifest does not have a package section");
                };

                manifests = Some(HashMap::from([(package.name.clone(), manifest)]));
            }

            let target_path = current_path.join("target");
            if target_dir.is_none() && target_path.is_dir() {
                target_dir = Some(target_path);
            }

            current_path = current_path.join("..");
        }

        Ok(Self {
            // These shouldn't panic since we're only exiting the loop if they're Some
            target_dir: target_dir.expect("No target path"),
            manifests: manifests.expect("No manifests"),
        })
    }
}
