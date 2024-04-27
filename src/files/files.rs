//! Utils to work with files
use crate::errors::CommonError as Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

/// Returns parent directory of path
///
/// # Errors
///
/// Returns [`Error::InvalidDirectoryPath`] if path was not found in system
///
/// Returns [`Error::EmptyInput`] if path does not have parent directory
pub fn get_directory(path: &Path) -> Result<PathBuf, Error> {
    match path.parent() {
        None => Err(Error::EmptyInput),
        Some(directory) => {
            if directory.exists() {
                Ok(directory.to_path_buf())
            } else {
                Err(Error::InvalidDirectoryPath {
                    path: directory.to_str().unwrap().to_string(),
                })
            }
        }
    }
}

/// Returns filename of the last element in path
///
/// # Errors
///
/// Returns [`Error::InvalidNamePattern`] if path does not have file_name
///
/// # Examples
/// ```
/// use std::path::{Path, PathBuf};
/// use mmv::files::files::get_filename;
/// let path = PathBuf::from("foo/bar.txt");
/// assert_eq!(
///     get_filename(path.as_path()).unwrap(),
///     String::from("bar.txt")   
/// );
/// ```
/// Error example:
/// ```
/// use std::path::{Path, PathBuf};
/// use mmv::files::files::get_filename;
/// let path = PathBuf::from("../../");
/// assert!(
///     get_filename(path.as_path()).is_err()
/// );
/// ```
/// more examples can be found in unit-testing
pub fn get_filename(path: &Path) -> Result<String, Error> {
    match path.file_name() {
        None => Err(Error::InvalidNamePattern),
        Some(filename) => Ok(String::from(filename.to_str().unwrap())),
    }
}

/// Gets filenames from the given directory
///
/// Returns only names of files, not directories
///
/// # Errors
///
/// Returns [`Error::IOError`] if could not open directory with [`fs::read_dir`]
pub fn get_filenames(path: &Path) -> Result<Vec<OsString>, Error> {
    let read_dir = fs::read_dir(path).map_err(|error| Error::IOError { error })?;
    Ok(read_dir
        .map(|entry_res| entry_res.unwrap())
        .filter(|file| file.metadata().unwrap().is_file())
        .map(|file| file.file_name())
        .collect())
}

#[inline]
/// Returns filename and working_directory of the path
pub fn split_path_to_filename(path: &str) -> Result<(PathBuf, String), Error> {
    let path = &PathBuf::from(path);
    Ok((get_directory(path)?, get_filename(path)?))
}

#[cfg(test)]
mod tests {
    use super::{get_directory, get_filename, get_filenames};
    use std::{ffi::OsString, path::PathBuf};

    use crate::files::tests::FileManager;

    #[test]
    fn test_get_directory() {
        let manager = FileManager::from("get_directory/");
        let check = |path_str: &str| {
            let path = PathBuf::from(path_str);
            let submanager = manager.create_subdirectory(path.as_path());
            submanager.create_files(&vec!["foo"]);
            assert_eq!(
                get_directory(submanager.get_directory().join("foo").as_path()).unwrap(),
                submanager.get_directory().to_owned()
            )
        };
        check("boo/");
        check("too/much/dirs/");
    }

    #[test]
    fn test_get_filename() {
        let manager = FileManager::from("get_filename");
        let check = |directory: &str, name: &str| {
            let submanager = manager.create_subdirectory(PathBuf::from(directory).as_path());
            submanager.create_files(&vec![name]);
            assert_eq!(
                get_filename(submanager.get_directory()).unwrap(),
                String::from(name)
            );
        };
        check("boo", "boo");
        check("mem/boo", "boo");
        check("too/much/dirs", "dirs");
    }

    #[test]
    fn test_get_filenames() {
        let manager = FileManager::from("get_filenames");
        let check = |subdirectory: &str, files: Vec<&str>| {
            let _submanager = manager.create_subdirectory(PathBuf::from(subdirectory).as_path());
            manager.create_files(&files);
            assert_eq!(
                get_filenames(manager.get_directory()).unwrap().sort(),
                files
                    .into_iter()
                    .map(|file| OsString::from(file))
                    .collect::<Vec<_>>()
                    .sort()
            )
        };
        check("boo/", vec!["foo", "moo"]);
    }
}
