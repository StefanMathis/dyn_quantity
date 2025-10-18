use std::env;
use std::error::Error;
use std::path::{PathBuf};
use std::process::Command;

const CRATE_NAME: &str = "dyn_quantity_from_str";
const LIB_NAME: &str = "libdyn_quantity_from_str.a";

fn main() -> Result<(), Box<dyn Error>> {
    // Only run staticlib logic if feature is NOT enabled
    #[cfg(not(feature = "no_static_lib"))]
    build_staticlib()?;

    Ok(())
}

#[cfg(not(feature = "no_static_lib"))]
fn build_staticlib() -> Result<(), Box<dyn Error>> {
    // Output directory for build artifacts
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let staticlib_target_dir = out_dir.join("dyn_quantity_staticlib_target");
    let staticlib_path = staticlib_target_dir.join("release").join(LIB_NAME);

    // Only rebuild the staticlib if it does not exist
    if !staticlib_path.exists() {
        println!("cargo:warning=Staticlib not found, building dyn_quantity_from_str...");

        let status = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--package",
                CRATE_NAME,
                "--target-dir",
                staticlib_target_dir.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to run cargo build");

        if !status.success() {
            panic!("Failed to build staticlib for {CRATE_NAME}");
        }
    } else {
        println!("cargo:warning=Staticlib already exists, skipping rebuild.");
    }

    // Link the staticlib
    println!(
        "cargo:rustc-link-search=native={}",
        staticlib_path.parent().unwrap().display()
    );
    println!("cargo:rustc-link-lib=static={}", CRATE_NAME);

    // Prevent rebuild if unnecessary
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
