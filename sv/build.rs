use std::{env, fs, path::{Path, PathBuf}, io::Write, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let targets = [
        (
            "Vpc",
            "src/processor/core/ifu/pc.sv",
            vec![],
        ),
        (
            "Valu",
            "src/processor/core/ieu/alu.sv",
            vec![],
        ),
        (
            "Vifu",
            "src/shim/ifu_shim.sv",
            vec!["src/processor/core/ifu", "src/bus"],
        ),
        (
            "Vlsu",
            "src/shim/lsu_shim.sv",
            vec!["src/processor/core/lsu", "src/bus"],
        ),
        (
            "Vjbu",
            "src/processor/core/ieu/jbu.sv",
            vec![],
        ),
        (
            "Vcore",
            "src/shim/core_shim.sv",
            vec![
                "src/processor/core/pipeline",
                "src/processor/core",
                "src/bus",
                "src/processor/core/hc",
                "src/processor/core/forward",
                "src/processor/core/idu",
                "src/processor/core/ieu",
                "src/processor/core/ifu",
                "src/processor/core/lsu",
                "src/processor/core/rf",
            ],
        ),
    ];

    for (prefix, path, deps) in targets.clone() {
        println!("cargo:rerun-if-changed={path}");
        let mut args = vec![
            "-Wall",
            "--cc",
            "src/processor/core/pipeline/pipeline.sv",
            "--trace",
            "--trace-structs",
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
            add_deps(Path::new(dep));
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

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut path = manifest_dir.parent().unwrap().to_path_buf();
    path.push("target");
    path.push("include.txt");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect(&format!("Failed to open path: {:?}", path));

    writeln!(file, "{}", out_dir.display()).unwrap();
}

fn add_deps(path: &Path) {

    // If the directory itself changes (files added/removed), rerun as well
    println!("cargo:rerun-if-changed={}", path.display());

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("sv") {
                if let Some(p) = path.to_str() {
                    println!("cargo:rerun-if-changed={}", p);
                }
            }
        }
    }
}
