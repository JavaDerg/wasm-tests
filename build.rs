fn main() {
    std::process::Command::new("cargo")
        .args(&[
            "+nightly",
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ])
        .current_dir("./hw/")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
