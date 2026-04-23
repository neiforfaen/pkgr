use cliclack::select;
use serde::Deserialize;
use serde_json::from_reader;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub struct Pkg {
    #[serde(default)]
    pub scripts: BTreeMap<String, serde::de::IgnoredAny>,
    #[serde(default, rename = "packageManager")]
    pub package_manager: String,
}

pub fn parse_pkg(path: &Path) -> Result<Pkg, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(from_reader(BufReader::new(file))?)
}

pub fn resolve_pkg_mgr(raw: &str) -> Result<String, Box<dyn std::error::Error>> {
    if raw.is_empty() {
        return select::<String>("packageManager not found, choose one")
            .item("pnpm".to_string(), "pnpm", "")
            .item("npm".to_string(), "npm", "")
            .item("yarn".to_string(), "yarn", "")
            .item("bun".to_string(), "bun", "")
            .interact()
            .map_err(Into::into);
    }
    Ok(raw
        .split_once('@')
        .map(|(mgr, _)| mgr)
        .unwrap_or(raw)
        .to_string())
}
