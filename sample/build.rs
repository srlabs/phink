use std::process::Command;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=flipper/lib.rs");
    println!("cargo::rerun-if-changed=bank/lib.rs");
    let output = Command::new("cargo")
        .args(["contract", "build", "--help", "--manifest-path", "flipper/Cargo.toml"])
        .output()
        .expect("Failed to execute command");

    println!("Output: {:?}", output.stdout);
}