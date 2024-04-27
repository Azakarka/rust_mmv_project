use std::io;

#[derive(thiserror::Error, Debug)]
/// Contains Errors that can be found in mmv project
pub enum CommonError {
    #[error("Invalid directory path {}", path)]
    InvalidDirectoryPath { path: String },
    #[error("Invalid name pattern")]
    InvalidNamePattern,
    #[error("Files for pattern \"{}\" not found", pattern)]
    NoMatchingFiles { pattern: String },
    #[error("Not able to replace existing file: {}", filename)]
    FilenameAlreadyExists { filename: String },
    #[error("{}", error)]
    IOError { error: io::Error },
    #[error("Empty input path")]
    EmptyInput,
    #[error("Invalid marker's value")]
    InvalidMarkersValue,
    #[error("Marker's value is greater than fragments count")]
    TooBigMarkerValue,
}
