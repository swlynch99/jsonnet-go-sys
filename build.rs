use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use target_lexicon::{Architecture, Mips32Architecture, Mips64Architecture, Triple};

fn main() {
    check_go_installed();

    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
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

    cmd.env("CGO_ENABLED", "1");

    let triple = Triple::from_str(&target).unwrap();
    if host != target {
        let detail = canonicalize_goarch(triple.architecture);

        cmd.env("GOOS", triple.operating_system.to_string());
        cmd.env("GOARCH", detail.arch);

        if let Some(arm) = detail.arm {
            cmd.env("GOARM", arm);
        }

        if let Some(amd64) = detail.amd64 {
            cmd.env("GOAMD64", amd64);
        }

        cmd.env(
            "CC",
            load_target_var("CC", &target).unwrap_or_else(|| "cc".into()),
        );
    }

    println!("{cmd:?}");

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

fn load_target_var(var: &str, target: &str) -> Option<OsString> {
    let fvar = format!("{var}_{target}");
    if let Some(var) = env::var_os(&fvar) {
        println!("cargo:rerun-if-env-changed={fvar}");
        return Some(var);
    }

    let fvar = format!("{var}_{}", target.replace("-", "_"));
    if let Some(var) = env::var_os(&fvar) {
        println!("cargo:rerun-if-env-changed={fvar}");
        return Some(var);
    }

    let fvar = format!("TARGET_{var}");
    if let Some(var) = env::var_os(&fvar) {
        println!("cargo:rerun-if-env-changed={fvar}");
        return Some(var);
    }

    if let Some(val) = env::var_os(var) {
        println!("cargo:rerun-if-env-changed={var}");
        return Some(val);
    }

    None
}

#[derive(Default)]
struct ArchDetail {
    // GOARCH env var
    arch: &'static str,

    // GOARM env var
    arm: Option<&'static str>,

    // GOAMD64 value
    amd64: Option<&'static str>,
}

fn canonicalize_goarch(arch: Architecture) -> ArchDetail {
    let arch = match arch {
        Architecture::X86_64 => "amd64",
        Architecture::X86_64h => {
            return ArchDetail {
                arch: "amd64",
                amd64: Some("v3"),
                ..Default::default()
            }
        }
        Architecture::X86_32(_) => "386",
        Architecture::Aarch64(_) => "arm64",
        Architecture::Arm(_) => "arm",
        Architecture::Powerpc64 => "ppc64",
        Architecture::Powerpc64le => "ppc64le",
        Architecture::Mips32(Mips32Architecture::Mips) => "mips",
        Architecture::Mips32(Mips32Architecture::Mipsel) => "mipsle",
        Architecture::Mips64(Mips64Architecture::Mips64) => "mips64",
        Architecture::Mips64(Mips64Architecture::Mips64el) => "mips64el",
        Architecture::Riscv64(_) => "riscv64",
        Architecture::S390x => "s390x",
        Architecture::Wasm32 => "wasm",
        _ => panic!("unsupported architecture `{arch}`"),
    };

    ArchDetail {
        arch,
        ..Default::default()
    }
}
