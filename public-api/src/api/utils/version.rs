use std::fs;
use toml::Value;


/// Retrieves the version number from the `Cargo.toml` file.
/// 
/// # Returns
/// 
/// The version number as a `String`.
pub fn version() -> String {
    let contents = fs::read_to_string("Cargo.toml").expect("Something went wrong reading the file");
    let parsed_value = contents.parse::<Value>().unwrap();
    let package = parsed_value.get("package").unwrap().get("version").unwrap().as_str().unwrap();

    package.to_string()
}


#[cfg(test)]
/// This module contains tests for the version function.
mod tests {
    use super::*;

    #[test]
    /// Test the version function by setting the current directory to "/tmp/",
    /// creating a dummy Cargo.toml file with a version number, and asserting
    /// that the version function returns the expected result.
    fn test_version() {
        std::env::set_current_dir("/tmp/").unwrap();

        let cargo_toml = r#"
            [package]
            name = "my-package"
            version = "1.0.0"
        "#;
        std::fs::write("/tmp/Cargo.toml", cargo_toml).unwrap();

        assert_eq!(version(), "1.0.0");
    }
}

