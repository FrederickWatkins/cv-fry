use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let targets = [
        ("Vpc", "src/processor/core/ifu/pc.sv", "test/pc/pc.cpp", vec![]),
        (
            "Valu",
            "src/processor/core/ieu/alu.sv",
            "test/alu/alu.cpp",
            vec![],
        ),
        (
            "Vifu",
            "test/ifu/ifu_shim.sv",
            "test/ifu/ifu.cpp",
            vec!["src/processor/core/ifu", "src/bus"],
        ),
        (
            "Vlsu",
            "test/lsu/lsu_shim.sv",
            "test/lsu/lsu.cpp",
            vec!["src/processor/core/lsu", "src/bus"],
        ),
        (
            "Vjbu",
            "src/processor/core/ieu/jbu.sv",
            "test/jbu/jbu.cpp",
            vec![],
        ),
        (
            "Vdecoder",
            "src/processor/core/decoder/decoder.sv",
            "test/decoder/decoder.cpp",
            vec!["src/processor/core/decoder/"],
        ),
        (
            "Vcore",
            "test/core/core_shim.sv",
            "test/core/core.cpp",
            vec!["src/bus", "src/processor/core", "src/processor/core/decoder", "src/processor/core/ieu", "src/processor/core/ifu", "src/processor/core/lsu", "src/processor/core/rf"],
        ),
    ];

    for (prefix, path, cpp, deps) in targets.clone() {
        println!("cargo:rerun-if-changed={path}");
        println!("cargo:rerun-if-changed={cpp}");
        let mut args = vec![
                "-Wall",
                "--cc",
                "--trace",
                "-Isrc",
                "--prefix",
                prefix,
                "-Mdir",
                out_dir.to_str().unwrap(),
                path,
            ];
        for dep in deps {
            args.push("-y");
            args.push(dep);
        }
        // 1. Run Verilator
        let status = Command::new("verilator")
            .args(args)
            .status()
            .expect("Failed to run Verilator");

        if !status.success() {
            panic!("Verilator execution failed");
        }
    }

    // 2. Automatically find ALL generated .cpp files
    // This solves the "undefined reference to Vpc___024root" error
    let mut c_builder = cc::Build::new();
    c_builder
        .cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .include("/usr/share/verilator/include")
        .include("/usr/share/verilator/include/vltstd")
        .include(&out_dir);

    // Collect all .cpp files in the out_dir
    for entry in fs::read_dir(&out_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            c_builder.file(path);
        }
    }

    // 3. Add the Verilator runtime files
    // Note: If you get "VlThreadPool" errors, we need verilated_threads.cpp too
    c_builder.file("/usr/share/verilator/include/verilated.cpp");
    c_builder.file("/usr/share/verilator/include/verilated_vcd_c.cpp");
    c_builder.file("/usr/share/verilator/include/verilated_threads.cpp"); // Add this for ThreadPool error

    for (_, _, shim, _) in targets {
        c_builder.file(shim);
    }

    c_builder.compile("verilated_cpp");

    let program_src_dir = PathBuf::from("programs/c");
    let linker_script = "programs/linker.ld";
    let entry_asm = "programs/entry.S";

    println!("cargo:rerun-if-changed={}", program_src_dir.display());
    println!("cargo:rerun-if-changed={}", linker_script);
    println!("cargo:rerun-if-changed={}", entry_asm);

    let entries = fs::read_dir(&program_src_dir).unwrap();
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
                    "--only-section=.text",
                    elf_out.to_str().unwrap(),
                    bin_out.to_str().unwrap(),
                ])
                .status()
                .expect("Failed to run llvm-objcopy");
        }
    }

    // 2. Run nested cargo build
    let status = Command::new("cargo")
        .current_dir("programs/rust")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .args(&[
            "build",
            "--release",
        ])
        .status()
        .expect("Failed to build Rust test payloads");

    if !status.success() {
        panic!("Nested Cargo build failed");
    }

    // 3. Locate the ELF and convert to BIN
    // Cargo places nested builds in a specific target folder
    let elf_path = PathBuf::from("programs/rust/target")
        .join("riscv32i-unknown-none-elf")
        .join("release/cv-fry-programs");
    
    let bin_out = out_dir.join("rust_test.bin");

    Command::new("llvm-objcopy")
        .args(&[
            "-O", "binary",
            "--only-section=.text",
            elf_path.to_str().unwrap(),
            bin_out.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to extract Rust binary");
}
