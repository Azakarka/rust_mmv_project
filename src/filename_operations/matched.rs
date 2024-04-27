//! Utils to match filenames by pattern
use std::ffi::OsString;

/// Filename data after matching
///
/// Contains original filaname and fragmented parts
///
/// # Examples
/// if filename = "foo_bar" and pattern = "\*_\*"
/// [`MatchedFilename`] will have the following structure:
/// ```
/// use std::ffi::OsString;
/// use mmv::filename_operations::matched::MatchedFilename;
/// MatchedFilename {
///     filename: OsString::from("foo_bar"),
///     fragments: vec![String::from("foo"), String::from("bar")]
/// };
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct MatchedFilename {
    /// Original filename
    pub filename: OsString,
    /// Extracted fragments according to pattern
    pub fragments: Vec<String>,
}

/// container (vector) of [`MatchedFilename`]
pub type MatchedFilenames = Vec<MatchedFilename>;

impl MatchedFilename {
    pub fn new(filename: OsString) -> Self {
        Self {
            filename,
            fragments: vec![],
        }
    }
}

/// Returns [`MatchedFilename`] according to pattern
///
/// Returns [`None`] if could not match pattern
///
/// # Examples
/// ```
/// use std::ffi::OsString;
/// use mmv::filename_operations::matched::{MatchedFilename, match_filename};
/// let filename = OsString::from("filename");
/// let pattern_good = "fi*me";
/// let pattern_bad = "if*em";
/// assert_eq!(
///     match_filename(&filename, pattern_good).unwrap(),
///     MatchedFilename {
///         filename: filename.clone(),
///         fragments: vec![String::from("lena")]
///     }
/// );
/// assert!(
///     match_filename(&filename, pattern_bad).is_none()
/// );
/// ```
pub fn match_filename(filename: &OsString, pattern: &str) -> Option<MatchedFilename> {
    let mut matched = MatchedFilename::new(filename.clone());
    let mut filename_suffix = filename.to_str().unwrap();
    let match_sequence: Vec<&str> = pattern.split("*").collect();

    for i in 0..match_sequence.len() {
        let mut string_to_match = match_sequence[i];
        filename_suffix = filename_suffix.strip_prefix(string_to_match)?;
        if i + 1 == match_sequence.len() {
            break;
        }
        string_to_match = match_sequence[i + 1];
        let mut split_index = filename_suffix.find(string_to_match)?;
        if string_to_match == "" && split_index == 0 {
            split_index = filename_suffix.len();
        }
        let mut fragment: &str = &"";
        (fragment, filename_suffix) = filename_suffix.split_at(split_index);
        matched.fragments.push(fragment.to_string());
    }
    if filename_suffix.is_empty() {
        Some(matched)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::filename_operations::matched::match_filename;
    use std::ffi::OsString;

    #[test]
    fn match_filename_ok() {
        fn match_filename_ok(pattern: &str, filename: &str, fragments: Vec<&str>) {
            let filename = OsString::from(filename.to_string());
            let matched = match_filename(&filename, pattern).unwrap();
            assert_eq!(matched.filename, filename);
            assert_eq!(matched.fragments, fragments);
        }
        match_filename_ok("*", "", vec![""]);
        match_filename_ok("_A*B_", "_AB_", vec![""]);
        match_filename_ok("*_*", "A_B", vec!["A", "B"]);
        match_filename_ok("A_*", "A_B", vec!["B"]);
        match_filename_ok("A_B", "A_B", vec![]);
        match_filename_ok("*_*_*", "ABA_CA_BA", vec!["ABA", "CA", "BA"]);
    }

    #[test]
    fn match_filename_none() {
        fn match_filename_none(pattern: &str, filename: &str) {
            let filename = OsString::from(filename.to_string());
            assert!(match_filename(&filename, pattern).is_none());
        }
        match_filename_none("*_*", "no underscore");
        match_filename_none("123", "1234");
        match_filename_none("123", "4123");
    }
}
