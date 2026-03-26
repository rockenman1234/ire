use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_nightly)");

    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    if let Ok(out) = Command::new(rustc).arg("--version").output() {
        let version = String::from_utf8_lossy(&out.stdout);
        if version.contains("nightly") {
            println!("cargo:rustc-cfg=has_nightly");
        }
    }
}