use std::{env, path::PathBuf};
use toml::Table;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut target_dir = manifest_dir.parent().unwrap().to_path_buf();
    target_dir.push("target");
    let toml_path = target_dir.join("payloads.toml");
    if toml_path.exists() {
        let content = std::fs::read_to_string(toml_path).unwrap();
        let toml_table = content.parse::<Table>().expect("Failed to parse toml");
        for (key, value) in toml_table {
            println!(
                "cargo:rustc-env=PAYLOAD_{}={}",
                key.to_uppercase(),
                target_dir.join(value.as_str().unwrap()).display()
            );
        }
    }
}
