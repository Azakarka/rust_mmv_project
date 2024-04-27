use serde;
use serde::Deserialize;
use serde_yaml;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct CommonSettings {
    working_directory: String,
    clean_test_data: bool,
}

const SETTINGS_PATH: &str = "tests/configs/common_settings.yaml";

fn load_settings() -> Result<CommonSettings, Box<dyn std::error::Error>> {
    let settings_file = fs::File::open(SETTINGS_PATH)?;
    let settings = serde_yaml::from_reader(settings_file)?;
    Ok(settings)
}

pub fn get_working_directory() -> PathBuf {
    let settings = load_settings().unwrap();
    PathBuf::from(settings.working_directory)
}

pub fn should_clean_test_data() -> bool {
    let settings = load_settings().unwrap();
    settings.clean_test_data
}
