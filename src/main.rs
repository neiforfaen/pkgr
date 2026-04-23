use cliclack::{intro, outro, select};
use console::style;
use pkgr::{parse_pkg, resolve_pkg_mgr};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, error::Error};

fn is_not_found(e: &Box<dyn Error>) -> bool {
    e.downcast_ref::<std::io::Error>()
        .map(|io| io.kind() == std::io::ErrorKind::NotFound)
        .unwrap_or(false)
}

fn load_pkg() -> Result<pkgr::Pkg, Box<dyn Error>> {
    let path = env::current_dir()?.join("package.json");
    match parse_pkg(&path) {
        Ok(pkg) => Ok(pkg),
        Err(e) if is_not_found(&e) => {
            outro(format!(
                "{} {}",
                style("✗ no package.json found in").red().bold(),
                style(path.display()).dim(),
            ))?;
            std::process::exit(0);
        }
        Err(e) => Err(e),
    }
}

fn select_script(names: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut prompt = select::<String>("choose cmd to run")
        .max_rows(8)
        .filter_mode();
    for name in &names {
        prompt = prompt.item(name.clone(), name, "");
    }
    match prompt.interact() {
        Ok(s) => Ok(s),
        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => std::process::exit(0),
        Err(e) => Err(e.into()),
    }
}

fn run_script(pkg_mgr: &str, script: &str) -> Result<(), Box<dyn Error>> {
    Err(Command::new(pkg_mgr).args(["run", script]).exec().into())
}

fn main() -> Result<(), Box<dyn Error>> {
    intro(format!("{}", style(" pkgr ").white().on_cyan().bright()))?;

    let pkg = load_pkg()?;
    let pkg_mgr = resolve_pkg_mgr(&pkg.package_manager)?;
    let names: Vec<String> = pkg.scripts.into_keys().collect();
    let selected = select_script(names)?;

    outro(format!(
        "{} {}",
        style("✓ running").green().bold(),
        style(&selected).dim(),
    ))?;

    run_script(&pkg_mgr, &selected)
}
