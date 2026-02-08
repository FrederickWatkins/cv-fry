use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // 1. Run Verilator
    let status = Command::new("verilator")
        .args([
            "-Wall", "--cc", "--trace",
            "-Isrc",
            "--prefix", "Vpc",
            "-Mdir", out_dir.to_str().unwrap(),
            "src/processor/core/ifu/pc.sv",
        ])
        .status()
        .expect("Failed to run Verilator");

    if !status.success() { panic!("Verilator execution failed"); }

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

    // 4. Add your bridge/shim if you wrote one
    c_builder.file("test/pc/pc.cpp");

    c_builder.compile("pc_hw");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=test/pc/pc.cpp");
}