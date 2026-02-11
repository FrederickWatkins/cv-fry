use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    let programs_src_dir = PathBuf::from("c");
    let linker_script = "linker.ld";
    let entry_asm = "entry.S";

    let entries = fs::read_dir(programs_src_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let elf_out = out_dir.join(format!("{}.elf", stem));
            let bin_out = out_dir.join(format!("{}.bin", stem));

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
                    "-T", linker_script,
                    "-o", elf_out.to_str().unwrap(),
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
                    "-O", "binary",
                    "--strip-all",
                    elf_out.to_str().unwrap(),
                    bin_out.to_str().unwrap(),
                ])
                .status()
                .expect("Failed to run llvm-objcopy");
        }
    }
}