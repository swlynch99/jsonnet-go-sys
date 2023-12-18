use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use target_lexicon::Triple;

fn main() {
    check_go_installed();

    // let host = env::var("HOST").unwrap();
    // let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=jsonnet");

    let mut cmd = Command::new("go");
    cmd.arg("build");
    cmd.arg("-buildmode=c-archive");
    cmd.arg("-o");
    cmd.arg(out_dir.join("libjsonnet.a"));
    cmd.arg("./c-bindings/c-bindings.go");
    cmd.arg("./c-bindings/handles.go");

    cmd.current_dir("go-jsonnet");

    let status = cmd.status().expect("failed to run `go build`");
    if !status.success() {
        panic!("`go build` exited with a non-zero exit code");
    }
}

fn check_go_installed() {
    let status = Command::new("go")
        .arg("version")
        .status()
        .expect("failed to execute `go version`");

    if !status.success() {
        panic!(
            "\
            `go version` exited with a non-zero exit code.\n\
            \n\
            You need to install go in order to build libjsonnet-go-sys.\
        "
        );
    }
}
