use crate::common::*;
use std::process::Command;

fn remove_dir_if_exists<P: AsRef<Path>>(dir: P) {
    if let Err(e) = fs::remove_dir_all(dir) {
        assert_eq!(
            e.kind(),
            std::io::ErrorKind::NotFound,
            "can't remove directory: {}",
            e
        )
    }
}

pub fn build_and_link() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = Path::new(&out_dir).join("libffi-src");
    let prefix = Path::new(&out_dir).join("libffi-root");
    let libdir = Path::new(&prefix).join("lib");
    let libdir64 = Path::new(&prefix).join("lib64");

    // Copy LIBFFI_DIR into src_dir to avoid an unnecessary build
    remove_dir_if_exists(&src_dir);
    remove_dir_if_exists(&prefix);
    fs::create_dir(&prefix).expect("can't write to OUT_DIR");
    run_command(
        "Copying libffi into the build directory",
        Command::new("cp").arg("-R").arg("libffi").arg(&src_dir),
    );

    // Generate configure, run configure, make, make install
    let mut build = autotools::Config::new(&src_dir);
    build
        .with("pic", None)
        .disable("docs", None)
        .out_dir(&prefix);

    // Install prefix is set in advance
    let _ = build.build();

    // Cargo linking directives
    println!("cargo:rustc-link-lib=static=ffi");
    println!("cargo:rustc-link-search=native={}", libdir.display());
    println!("cargo:rustc-link-search=native={}", libdir64.display());
}

pub fn probe_and_link() {
    println!("cargo:rustc-link-lib=dylib=ffi");
}
