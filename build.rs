use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const CRATE_NAME: &str = "dyn_quantity_from_str";

#[cfg(any(feature = "no_static_lib", not(feature = "from_str")))]
fn main() -> Result<(), Box<dyn Error>> {
    // If no_static_lib is enabled, we don't link to anything
    Ok(())
}

#[cfg(all(not(feature = "no_static_lib"), feature = "from_str"))]
fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let crate_dir = manifest_dir.join("dyn_quantity_from_str");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let crate_target_dir = out_dir.join("dyn_quantity_staticlib_target");
    let release_dir = crate_target_dir.join("release").join("deps");

    println!(
        "cargo:rerun-if-changed={}",
        crate_dir.join("src/lib.rs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        crate_dir.join("Cargo.toml").display()
    );

    let (lib_path, _orig_name) = match find_static_lib(&release_dir) {
        Some(result) => result,
        None => {
            println!("Static library not found, building {}...", CRATE_NAME);

            let status = Command::new("cargo")
                .args([
                    "build",
                    "--release",
                    "--package",
                    CRATE_NAME,
                    "--target-dir",
                    &crate_target_dir.to_string_lossy(),
                    "--lib",
                ])
                .status()?;

            if !status.success() {
                panic!("Failed to build {CRATE_NAME} static library");
            }

            find_static_lib(&release_dir).expect("Staticlib not found after building!")
        }
    };

    // Normalized name: libdyn_quantity_from_str.a
    let normalized_path = lib_path
        .parent()
        .unwrap()
        .join(format!("lib{CRATE_NAME}.a"));

    if !normalized_path.exists() {
        fs::copy(&lib_path, &normalized_path)?;
        println!(
            "Copied staticlib to normalized name: {}",
            normalized_path.display()
        );
    }

    // Link the normalized version
    link_lib(&normalized_path);

    Ok(())
}

#[cfg(not(feature = "no_static_lib"))]
fn find_static_lib(release_dir: &Path) -> Option<(PathBuf, String)> {
    let entries = fs::read_dir(release_dir).ok()?;
    for entry in entries {
        if let Ok(entry) = entry {
            let fname = entry.file_name().to_string_lossy().to_string();
            if fname.starts_with("libdyn_quantity_from_str-") && fname.ends_with(".a") {
                return Some((entry.path(), fname));
            }
        }
    }
    None
}

#[cfg(not(feature = "no_static_lib"))]
fn link_lib(lib_path: &Path) {
    let dir = lib_path.parent().expect("No parent dir for .a file");
    println!("cargo:rustc-link-search=native={}", dir.display());

    // Link with the normalized name (without hash)
    println!("cargo:rustc-link-lib=static={}", CRATE_NAME);
}
