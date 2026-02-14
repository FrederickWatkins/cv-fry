use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut out_dir = manifest_dir.parent().unwrap().to_path_buf();
    out_dir.push("target");
    let mut c_out_dir = out_dir.clone();
    c_out_dir.push("payload_c");
    fs::create_dir_all(&c_out_dir).unwrap();

    let programs_src_dir = PathBuf::from("c");
    let linker_script = "linker.ld";
    let entry_asm = "entry.S";

    let mut mapping = Vec::new();

    let entries = fs::read_dir(programs_src_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let elf_out = c_out_dir.join(format!("{}.elf", stem));
            let bin_out = c_out_dir.join(format!("{}.bin", stem));
            mapping.push((stem.to_string(), bin_out.clone()));

            // Compile C + Entry.S -> ELF
            let clang_status = Command::new("clang")
                .args(&[
                    "--target=riscv32",
                    "-march=rv32i",
                    "-mabi=ilp32",
                    "-ffreestanding",
                    "-nostdlib",
                    "-static",
                    "-fuse-ld=lld",
                    "-T",
                    linker_script,
                    "-o",
                    elf_out.to_str().unwrap(),
                    entry_asm,
                    path.to_str().unwrap(),
                ])
                .status()
                .expect("Failed to run clang");

            if !clang_status.success() {
                panic!("Failed to compile test program: {}", stem);
            }

            // ELF -> BIN
            Command::new("llvm-objcopy")
                .args(&[
                    "-O",
                    "binary",
                    "--strip-all",
                    elf_out.to_str().unwrap(),
                    bin_out.to_str().unwrap(),
                ])
                .status()
                .expect("Failed to run llvm-objcopy");
        }
    }

    // 2. Run nested cargo build
    let status = Command::new("cargo")
        .current_dir("rust")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .args(&[
            "build",
            "--release",
            "--target-dir",
            &format!("{}", out_dir.display()),
        ])
        .status()
        .expect("Failed to build Rust test payloads");

    if !status.success() {
        panic!("Nested Cargo build failed");
    }

    // 3. Locate the ELF and convert to BIN
    // Cargo places nested builds in a specific target folder
    let elf_path = out_dir.clone()
        .join("riscv32im-unknown-none-elf")
        .join("release/cv-fry-payload-rs");

    let bin_out = out_dir.join("rust_test.bin");

    Command::new("llvm-objcopy")
        .args(&[
            "-O",
            "binary",
            "--strip-all",
            elf_path.to_str().unwrap(),
            bin_out.clone().to_str().unwrap(),
        ])
        .status()
        .expect("Failed to extract Rust binary");

    mapping.push(("cv-fry-payload-rs".to_string(), bin_out));

    let mut toml_output = String::from("# Auto-generated payload mapping\n\n");
    for (name, abs_path) in mapping {
        // Get path relative to the target directory
        let rel_path = abs_path.strip_prefix(&out_dir).unwrap_or(&abs_path);
        toml_output.push_str(&format!(
            "{} = \"{}\"\n",
            name,
            rel_path.to_str().unwrap().replace("\\", "/") // Use forward slashes for cross-platform TOML
        ));
    }

    fs::write(out_dir.join("payloads.toml"), toml_output).expect("Failed to write mapping");
}
