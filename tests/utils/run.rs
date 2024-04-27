use mmv::errors::CommonError as Error;
use mmv::run;
use mmv::Arguments;
use std::fs::read;
use std::path::PathBuf;
use std::time::SystemTime;

use super::file_manager::FileManager;

#[derive(Debug, PartialEq)]
pub struct FileMeta {
    created: SystemTime,
    content: Vec<u8>,
}

pub struct RunParams<'a> {
    pub input_directory: &'a PathBuf,
    pub output_directory: &'a PathBuf,
    pub input_pattern: &'a str,
    pub output_pattern: &'a str,
    pub filenames_to_match: Vec<&'a str>,
    pub modified_filenames: Vec<&'a str>,
    pub force: Option<bool>,
}

pub fn check_run(params: RunParams) -> Result<(), Error> {
    assert_eq!(
        params.filenames_to_match.len(),
        params.modified_filenames.len()
    );
    let arguments = Arguments {
        input_template: String::from(
            params
                .input_directory
                .join(params.input_pattern)
                .to_str()
                .unwrap(),
        ),
        output_template: String::from(
            params
                .output_directory
                .join(params.output_pattern)
                .to_str()
                .unwrap(),
        ),
        force: params.force.unwrap_or(false),
    };
    let mut metadata: Vec<FileMeta> = vec![];
    for filename in params.filenames_to_match {
        let path = params.input_directory.join(filename);
        metadata.push(FileMeta {
            created: path.metadata().unwrap().created().unwrap(),
            content: read(path).unwrap(),
        });
    }
    run(arguments)?;
    for i in 0..params.modified_filenames.len() {
        let filename = &params.modified_filenames[i];
        let path = params.output_directory.join(filename);
        assert_eq!(
            metadata[i],
            FileMeta {
                created: path.metadata().unwrap().created().unwrap(),
                content: read(path).unwrap(),
            }
        );
    }
    Ok(())
}

pub fn do_run(
    manager: &FileManager,
    filenames_to_match: Vec<&str>,
    modified_filenames: Vec<&str>,
    input_pattern: &str,
    output_pattern: &str,
) -> Result<(), Error> {
    manager.clear_directory();
    manager.create_files(&filenames_to_match);
    check_run(RunParams {
        input_directory: manager.get_directory(),
        output_directory: manager.get_directory(),
        input_pattern,
        output_pattern,
        filenames_to_match,
        modified_filenames,
        force: Some(false),
    })?;
    Ok(())
}
