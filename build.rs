fn main() {
    // Skip README generation on docs.rs
    if std::env::var("DOCS_RS").as_deref() == Ok("1") {
        return;
    }

    let readme_template =
        std::fs::read_to_string("README.template.md").expect("Failed to read README template");

    let versioned_readme = readme_template.replace(
        "{{VERSION}}",
        &std::env::var("CARGO_PKG_VERSION").expect("version is available in build.rs"),
    );

    // Generate README.md for docs.rs
    let docsrs_readme = versioned_readme.clone();
    let _ = std::fs::write("README.md", docsrs_readme);
}
