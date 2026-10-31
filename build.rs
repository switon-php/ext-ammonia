use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let root = Path::new(&manifest);
    let head = root.join(".git/HEAD");
    if head.exists() {
        println!("cargo:rerun-if-changed={}", head.display());
    }
    let tags = root.join(".git/refs/tags");
    if tags.exists() {
        println!("cargo:rerun-if-changed={}", tags.display());
    }

    let version = extension_version(root);
    println!("cargo:rustc-env=EXT_VERSION={version}");
    let stamp = root.join("target/ext-version");
    if let Some(parent) = stamp.parent() {
        let _ = std::fs::create_dir_all(parent);
        let _ = std::fs::write(stamp, &version);
    }
}

fn extension_version(root: &Path) -> String {
    if let Some(tag) = git(root, &["describe", "--tags", "--exact-match"]) {
        return strip_tag(&tag);
    }
    if let Some(described) = git(root, &["describe", "--tags", "--always"]) {
        if described.contains('.') {
            return strip_tag(&described);
        }
    }
    std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string())
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn strip_tag(tag: &str) -> String {
    tag.strip_prefix('v')
        .filter(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
        .unwrap_or(tag)
        .to_string()
}
