fn main() {
    if !std::process::Command::new("cargo")
        .args(&[
            "+nightly",
            "build",
            "--target",
            "wasm32-wasi",
            "--release",
        ])
        .current_dir("./hw/")
        .status()
        .unwrap()
        .success() {
        panic!("Failed to compile hw");
    }
    println!("cargo:rerun-if-changed=hw/");
}
