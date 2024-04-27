use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use super::utils::{create_directory, get_path, TestName};

#[path = "../config.rs"]
mod config;
use config::should_clean_test_data;
pub struct FileManager {
    working_directory: PathBuf,
}

impl FileManager {
    pub fn new(working_directory: PathBuf) -> Self {
        let instance = FileManager { working_directory };
        if instance.working_directory.exists() {
            instance.clear_directory();
        } else {
            create_directory(&instance.working_directory);
        }
        instance
    }

    pub fn from_test_name(test_name: TestName) -> Self {
        Self::new(get_path(test_name))
    }

    pub fn get_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    pub fn clear_directory(&self) {
        if self.working_directory.exists() {
            assert!(fs::remove_dir_all(&self.working_directory).is_ok());
        }
        assert!(fs::create_dir_all(&self.working_directory).is_ok());
    }

    pub fn create_subdirectory(&self, path: &Path) -> Self {
        Self::new(self.working_directory.join(path))
    }

    pub fn create_files(&self, filenames: &Vec<&str>) {
        for filename in filenames {
            let new_path = self.working_directory.join(filename);
            assert!(File::create(&new_path).is_ok());
            assert!(fs::write(&new_path, format!("Original name: {}", filename)).is_ok());
        }
    }
}

impl Drop for FileManager {
    fn drop(&mut self) {
        if should_clean_test_data() {
            self.clear_directory()
        }
    }
}
