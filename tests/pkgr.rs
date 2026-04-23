use pkgr::{parse_pkg, resolve_pkg_mgr};
use std::path::Path;

fn write_pkg(dir: &Path, content: &str) -> std::path::PathBuf {
    let path = dir.join("package.json");
    std::fs::write(&path, content).unwrap();
    path
}

#[test]
fn resolve_pkg_mgr_strips_version() {
    assert_eq!(resolve_pkg_mgr("pnpm@9.0.0").unwrap(), "pnpm");
}

#[test]
fn resolve_pkg_mgr_keeps_plain_name() {
    assert_eq!(resolve_pkg_mgr("npm").unwrap(), "npm");
}

#[test]
fn parse_pkg_reads_scripts() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_pkg(dir.path(), r#"{"scripts":{"build":"tsc","test":"jest"}}"#);
    let pkg = parse_pkg(&path).unwrap();
    let names: Vec<&String> = pkg.scripts.keys().collect();
    assert_eq!(names, ["build", "test"]);
}

#[test]
fn parse_pkg_reads_package_manager() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_pkg(dir.path(), r#"{"packageManager":"pnpm@9.1.0"}"#);
    let pkg = parse_pkg(&path).unwrap();
    assert_eq!(pkg.package_manager, "pnpm@9.1.0");
}

#[test]
fn parse_pkg_defaults_empty_on_missing_fields() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_pkg(dir.path(), r#"{}"#);
    let pkg = parse_pkg(&path).unwrap();
    assert!(pkg.scripts.is_empty());
    assert_eq!(pkg.package_manager, "");
}

#[test]
fn parse_pkg_missing_file_returns_error() {
    let result = parse_pkg(Path::new("/nonexistent/package.json"));
    assert!(result.is_err());
}
