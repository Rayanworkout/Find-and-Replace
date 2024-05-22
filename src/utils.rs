use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub trait PathExt {
    fn is_directory(&self) -> Result<bool>;
    fn should_omit(&self, omit: &[PathBuf]) -> bool;
}

impl PathExt for PathBuf {
    /// Check if a given path is a directory
    /// Returns true if path is directory, otherwise false
    fn is_directory(&self) -> Result<bool> {
        let metadata = fs::metadata(self).with_context(|| {
            format!(
                "Failed to check if the following path is a directory: {:?}",
                self
            )
        })?;

        Ok(metadata.is_dir())
    }

    /// Function to check if a specific path should be omitted
    /// We iterate over the paths to omit and check if the current path
    /// starts with any of the paths to omit
    /// Returns true if the path should be omitted, otherwise false
    fn should_omit(&self, omit: &[PathBuf]) -> bool {
        omit.iter().any(|omit_path| self.starts_with(omit_path))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_directory() {
        let path = PathBuf::from("src");
        assert!(path.is_directory().unwrap());
    }

    #[test]
    fn test_should_omit() {
        let path = PathBuf::from("src");
        let omit = vec![PathBuf::from("src")];
        assert!(path.should_omit(&omit));
    }

    #[test]
    fn test_should_not_omit() {
        let path = PathBuf::from("src");
        let omit = vec![PathBuf::from("tests")];
        assert!(!path.should_omit(&omit));
    }
}
