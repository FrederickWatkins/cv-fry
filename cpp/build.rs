use std::{env, fs, path::PathBuf, io::{BufRead, BufReader}};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut path = manifest_dir.parent().unwrap().to_path_buf();
    path.push("target");
    path.push("include.txt");

    let file = fs::OpenOptions::new()
        .read(true)
        .open(&path)
        .expect(&format!("Failed to open path: {:?}", path));

    let out_dir = BufReader::new(file).lines().next().expect("Failed to read include dir").expect("Failed to read include dir");

    let mut c_builder = cc::Build::new();
    c_builder
        .cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .include("/usr/share/verilator/include")
        .include("/usr/share/verilator/include/vltstd")
        .include(&out_dir);

    for entry in fs::read_dir("src").unwrap() {
        let entry = entry.unwrap();
        let path = &entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            c_builder.file(path);
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Collect all .cpp files in the out_dir
    for entry in fs::read_dir(&out_dir).unwrap() {
        let entry = entry.unwrap();
        let path = &entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            c_builder.file(path);
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    c_builder.file("/usr/share/verilator/include/verilated.cpp");
    c_builder.file("/usr/share/verilator/include/verilated_vcd_c.cpp");
    c_builder.file("/usr/share/verilator/include/verilated_threads.cpp"); // Add this for ThreadPool error

    c_builder.compile("verilated_cpp");
}
