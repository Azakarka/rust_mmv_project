use std::fs;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};

fn get_working_directory() -> PathBuf {
    PathBuf::from("./tests/test_data/unit_tests")
}

pub struct FileManager {
    working_directory: PathBuf,
    relative_name: PathBuf,
}

impl FileManager {
    pub fn new(working_directory: PathBuf) -> Self {
        let instance = FileManager {
            working_directory: get_working_directory().join(&working_directory),
            relative_name: working_directory,
        };
        if instance.working_directory.exists() {
            instance.clear_directory();
        } else {
            create_dir_all(instance.working_directory.as_path()).unwrap();
        }
        instance
    }

    pub fn from(working_directory: &str) -> Self {
        Self::new(PathBuf::from(working_directory))
    }

    pub fn get_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    pub fn create_subdirectory(&self, path: &Path) -> Self {
        Self::new(self.relative_name.join(path))
    }

    pub fn clear_directory(&self) {
        if self.working_directory.exists() {
            assert!(fs::remove_dir_all(&self.working_directory).is_ok());
        }
        assert!(fs::create_dir_all(&self.working_directory).is_ok());
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
        self.clear_directory()
    }
}
