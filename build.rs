use std::process::{Command, Stdio};
use std::env;

fn main() {
    let ret = Command::new("make")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("make missing on the system");
    assert!(ret.success());
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-flags=-L {} -l static=parser", out_dir);
}
