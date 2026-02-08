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
    ];

    for (prefix, path, _, deps) in targets.clone() {
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

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=test/pc/pc.cpp");
}
