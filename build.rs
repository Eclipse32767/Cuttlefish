use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src");
    Command::new("xtr").arg("-o").arg("locale-examples/cuttlefish.po").arg("src/cuttlefish.rs").output().unwrap();
    Command::new("xtr").arg("-o").arg("locale-examples/sunfish.po").arg("src/sunfish.rs").output().unwrap();
}