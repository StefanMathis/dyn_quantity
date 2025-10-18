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
const LIB_NAME: &'static str = "libdyn_quantity_from_str.a";

#[cfg(not(feature = "no_static_lib"))]
fn inner() -> Result<(), Box<dyn Error>> {
    use std::{
        env::{self, set_current_dir},
        fs::{self, create_dir},
        io::{self},
        path::Path,
    };

    use std::path::PathBuf;

    fn build_lib(crate_dir: &Path) -> Result<(), Box<dyn Error>> {
        // ***************************************************************************

        #[cfg(not(feature = "no_static_lib"))]
        fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
            fs::create_dir_all(&dst)?;
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let ty = entry.file_type()?;
                if ty.is_dir() {
                    copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
                } else {
                    fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
                }
            }
            Ok(())
        }

        #[cfg(not(feature = "no_static_lib"))]
        fn copy_from_crate_to_lib_builder(
            crate_dir: &Path,
            ext_lib_crate_dir: &Path,
            file_path: &[&str],
        ) -> io::Result<()> {
            let mut src = crate_dir.to_path_buf();
            let mut dst = ext_lib_crate_dir.to_path_buf();
            for item in file_path {
                src.push(item);
                dst.push(item);
            }
            fs::copy(src.as_path(), dst.as_path())?;
            return Ok(());
        }

        // ===================================================================================

        // Create a new crate from the template
        let mut template_dir = crate_dir.to_path_buf();
        template_dir.push("dyn_quantity_from_str_template");

        let mut ext_lib_crate_dir = crate_dir.to_path_buf();
        ext_lib_crate_dir.push("dyn_quantity_from_str");
        let _ = create_dir(ext_lib_crate_dir.as_path());

        // If the crate exists, remove it
        if ext_lib_crate_dir.exists() {
            fs::remove_dir_all(ext_lib_crate_dir.as_path()).map_err(Box::new)?;
        }

        copy_dir_all(template_dir.as_path(), ext_lib_crate_dir.as_path()).map_err(Box::new)?;

        // Copy some files from the main crate which are needed to build the library crate
        copy_from_crate_to_lib_builder(crate_dir, &ext_lib_crate_dir, &["src", "lib.rs"])
            .map_err(Box::new)?;
        copy_from_crate_to_lib_builder(
            crate_dir,
            &ext_lib_crate_dir,
            &["src", "from_str", "from_str_impl.rs"],
        )
        .map_err(Box::new)?;

        // Build the library
        let current_dir = env::current_dir().map_err(Box::new)?;

        set_current_dir(ext_lib_crate_dir.as_path()).map_err(Box::new)?;

        // Rename Cargo.template to Cargo.toml
        let src = "Cargo.template";
        let dst = "Cargo.toml";
        if !Path::new(dst).exists() {
            fs::copy(src, dst).expect("Failed to copy Cargo.template to Cargo.toml");
        }

        let _ = std::process::Command::new("cargo")
            .args(["build", "--package", "dyn_quantity_from_str", "--release"])
            .output()
            .map_err(Box::new)?;

        // Move the library into the OUT_DIR of this crate
        let mut src = crate_dir.to_path_buf();
        src.push("dyn_quantity_from_str");
        src.push("target");
        src.push("release");
        src.push(LIB_NAME);

        // Get the path to the build output directory
        let out_dir = env::var("OUT_DIR").unwrap();
        let dst = Path::new(&out_dir).join("libdyn_quantity_from_str.a");

        fs::copy(src.as_path(), dst.as_path()).map_err(Box::new)?;

        // Go back to the current working directory
        set_current_dir(current_dir.as_path()).map_err(Box::new)?;

        // Delete the entire ext_lib_crate
        fs::remove_dir_all(ext_lib_crate_dir.as_path()).map_err(Box::new)?;

        return Ok(());
    }

    // ===================================================================

    // Provide search path
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=dyn_quantity_from_str");

    // Check if the lib is already compiled. If not, build the library
    let crate_dir = PathBuf::from(out_dir);
    let mut lib_dir = crate_dir.clone();
    lib_dir.push(LIB_NAME);

    if !fs::exists(lib_dir.as_path()).map_err(Box::new)? {
        return build_lib(crate_dir.as_path());
    } else {
        return Ok(());
    }
}
