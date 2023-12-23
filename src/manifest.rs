use anyhow::{anyhow, bail, Result};
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
        while target_dir.is_none() || manifests.is_none() {
            if current_path.parent().expect("No parent") == current_path {
                if manifests.is_none() {
                    bail!("Not in a Cargo project directory");
                } else {
                    bail!("Hit root searching for target directory");
                }
            }

            let manifest_path = current_path.join("Cargo.toml");
            if manifests.is_none() && manifest_path.is_file() {
                let manifest = Manifest::from_path(manifest_path)?;

                if let Some(workspace) = manifest.workspace {
                    let mut map = HashMap::new();
                    for member in workspace.members {
                        let path = current_path.join(member);
                        map.insert(
                            path.file_name()
                                .ok_or_else(|| {
                                    anyhow!("Couldn't extract file name from '{}'", path.display())
                                })?
                                .to_string_lossy()
                                .into_owned(),
                            Manifest::from_path(path.join("Cargo.toml"))?,
                        );
                    }
                    manifests = Some(map);
                } else if let Some(ref package) = manifest.package {
                    manifests = Some(HashMap::from([(package.name.clone(), manifest)]));
                } else {
                    bail!("Cargo.toml does not have a package section");
                };
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
