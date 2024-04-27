//! Move and rename files by filename pattern
//!
//! A tool to move multiple files by filename a simple regex-like pattern (only '*' available)
//! # Example
//! ```[bash]
//! mmv ./*_*.txt -> ./#2_#1.tx
//! ```
//! All files in local directory that match the pattern will be changed accordingly
//! ```[text]
//! foo_bar.text -> bar_foo.txt
//! ```

pub mod errors;
pub mod filename_operations;
pub mod files;

use crate::filename_operations::operations::match_and_modify_filenames;
use crate::files::files::split_path_to_filename;
use crate::files::r#move::move_files;
use clap::Parser;
use errors::CommonError;
use files::files::get_filenames;

#[derive(Parser, Debug)]
#[clap(
    version,
    about = "Simple program to rename multiple files by a template"
)]
/// Deserialized command-line arguments
pub struct Arguments {
    /// Path to directory where files lie & template of changes
    pub input_template: String,

    /// Template of future file names
    pub output_template: String,

    #[clap(short, long)]
    /// Override existing files
    pub force: bool,
}

/// Starts the whole program
/// - Reads filenames from the directory using [`get_filenames`]
/// - Changes filenames according to the pattern using [`match_and_modify_filenames`]
/// - Renames and/or moves files using [`move_files`]
pub fn run(arguments: Arguments) -> Result<(), CommonError> {
    let (input_directory, input_pattern) = split_path_to_filename(&arguments.input_template)?;
    let (output_directory, output_pattern) = split_path_to_filename(&arguments.output_template)?;

    let filenames = get_filenames(&input_directory)?;
    let modified_filenames =
        match_and_modify_filenames(filenames, &input_pattern, &output_pattern)?;

    move_files(
        modified_filenames,
        &input_directory,
        &output_directory,
        arguments.force,
    )
}
