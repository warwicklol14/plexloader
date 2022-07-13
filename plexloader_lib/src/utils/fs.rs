use std::path::{Path};
use std::fs::create_dir_all;
use crate::DirectoryCreationError;

pub fn create_dir(dir_path: &Path) -> Result<(), DirectoryCreationError> {
    if !dir_path.exists() {
        create_dir_all(dir_path)?;
    }
    Ok(())
}
