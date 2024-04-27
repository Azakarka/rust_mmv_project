use std::fs::create_dir_all;
use std::path::PathBuf;

#[path = "../config.rs"]
mod config;
use config::get_working_directory;

pub enum TestName {
    OkSimple,
    OkSpecial,
    NoMatchedFiles,
    FileExistsNoForce,
    FileExistsForceEnabled,
    DifferentDirectories,
}

pub fn get_path(test_name: TestName) -> PathBuf {
    let working_directory = get_working_directory();
    let test_subdirectory = match test_name {
        TestName::OkSimple => "ok_simple/",
        TestName::OkSpecial => "ok_special/",
        TestName::NoMatchedFiles => "no_matched_files/",
        TestName::FileExistsNoForce => "file_exists_no_force/",
        TestName::FileExistsForceEnabled => "file_exists_force_enabled/",
        TestName::DifferentDirectories => "different_directories/",
    };
    let final_directory = working_directory.join(test_subdirectory);
    final_directory
}

pub fn create_directory(path: &PathBuf) {
    create_dir_all(path).unwrap();
}
