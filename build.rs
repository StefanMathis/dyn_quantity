use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    return inner();
}

#[cfg(feature = "no_static_lib")]
fn inner() -> Result<(), Box<dyn Error>> {
    // Do nothing
    return Ok(());
}

#[cfg(not(feature = "no_static_lib"))]
fn inner() -> Result<(), Box<dyn Error>> {
    use std::env;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    // Constants
    const CRATE_NAME: &'static str = "dyn_quantity_from_str";
    const LIB_NAME: &'static str = "libdyn_quantity_from_str.a";

    let crate_static_lib_dir = Path::new("dyn_quantity_from_str");

    // build.rs should not build to any directory except OUT_DIR
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_static_lib_target_dir = out_dir.join("crate_static_lib_target");

    // The static library is expected at this directory.
    let static_lib_path = crate_static_lib_target_dir.join("release").join(LIB_NAME);

    // Re-run build.rs if dyn_quantity_from_str source changes
    println!("cargo:rerun-if-changed=dyn_quantity_from_str/src/lib.rs");
    println!("cargo:rerun-if-changed=dyn_quantity_from_str/Cargo.toml");

    // Build dyn_quantity_from_str as a static library
    if !static_lib_path.exists() {
        println!(
            "Static library not found at {:?}, building dyn_quantity_from_str...",
            static_lib_path
        );
        
        let status = Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "--manifest-path",
                crate_static_lib_dir.join("Cargo.toml").to_str().unwrap(),
                "--target-dir",
                crate_static_lib_target_dir.to_str().unwrap(),
                "--lib",
            ])
            .status()
            .map_err(Box::new)?;

        if !status.success() {
            panic!("Failed to build {CRATE_NAME} static library");
        }
    } else {
        println!(
            "Static library found at {:?}, skipping build.",
            static_lib_path
        );
    }

    println!(
        "cargo:rustc-link-search=native={}",
        static_lib_path.parent().unwrap().display()
    );
    println!("cargo:rustc-link-lib=static={}", CRATE_NAME);

    return Ok(());
}
