//! Utils to move files after pattern-matching
use crate::errors::CommonError as Error;
use crate::filename_operations::modified::ModifiedFilenames;
use std::fs;
use std::path::Path;

/// Returns [`Ok`] if paths of [`ModifiedFilenames`] does **NOT** exist
///
/// # Errors
///
/// Returns [`Error::FilenameAlreadyExists`] if path exists
fn check_if_files_exist(modified: &ModifiedFilenames, directory_out: &Path) -> Result<(), Error> {
    for filename in modified {
        let new_path = directory_out.join(&filename.changed);
        if new_path.exists() {
            return Err(Error::FilenameAlreadyExists {
                filename: filename.changed.clone(),
            });
        }
    }
    Ok(())
}

/// Moves [`ModifiedFilenames`] from given direcrories
///
/// # Errors
///
/// If force flag is disabled returns [`Error::FilenameAlreadyExists`] if files already exist in system
///
/// Propagates [`Error::IOError`] from [`fs::rename`]
pub fn move_files(
    modified: ModifiedFilenames,
    directory_in: &Path,
    directory_out: &Path,
    force: bool,
) -> Result<(), Error> {
    if !force {
        check_if_files_exist(&modified, directory_out)?;
    }
    for filename in modified {
        let input_path = directory_in.join(Path::new(&filename.origin));
        let output_path = directory_out.join(Path::new(&filename.changed));
        fs::rename(input_path, output_path).map_err(|error| Error::IOError { error })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::move_files;
    use std::{ffi::OsString, path::PathBuf};

    use crate::{filename_operations::modified::ModifiedFilename, files::tests::FileManager};

    fn move_files_check(
        manager_in: &FileManager,
        manager_out: &FileManager,
        origin: &str,
        changed: &str,
        force: bool,
    ) {
        manager_in.clear_directory();
        manager_out.clear_directory();
        manager_in.create_files(&vec![origin]);
        assert!(move_files(
            vec![ModifiedFilename {
                origin: OsString::from(origin),
                changed: String::from(changed)
            }],
            &manager_in.get_directory(),
            &manager_out.get_directory(),
            force
        )
        .is_ok());
        assert!(manager_out.get_directory().join(changed).exists());
    }

    #[test]
    fn move_files_ok() {
        let manager = FileManager::from("move_files_ok/");
        let check = |origin: &str, changed: &str| {
            move_files_check(&manager, &manager, origin, changed, true);
        };
        check("228", "1337");
        check("boba", "boba");
        check("😁😁😁", "😡😡😡");
    }

    #[test]
    #[should_panic]
    fn move_files_no_force() {
        let manager = FileManager::from("move_files_force/");
        move_files_check(&manager, &manager, "file exists", "file exists", false);
    }

    #[test]
    fn move_files_different_directories() {
        let manager = FileManager::from("move_files_different_directories/");
        let manager_in = manager.create_subdirectory(PathBuf::from("in").as_path());
        let manager_out = manager.create_subdirectory(PathBuf::from("out.").as_path());
        let check = |origin: &str, changed: &str| {
            move_files_check(&manager_in, &manager_out, origin, changed, false);
        };
        check("aboba", "amoga");
        check("aboba", "aboba");
        check("***", "****");
        check("😁😁😁", "😡😡😡");
    }
}
