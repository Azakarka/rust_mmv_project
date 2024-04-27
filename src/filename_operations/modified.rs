//! Utils to modify filenames by pattern
use super::matched::MatchedFilename;
use crate::errors::CommonError as Error;
use regex::{Captures, Regex};
use std::ffi::OsString;

pub struct ModifiedFilename {
    pub origin: OsString,
    pub changed: String,
}

/// container(vector) of [`ModifiedFilename`]
pub type ModifiedFilenames = Vec<ModifiedFilename>;

/// Returns Ok(()) if all markers in pattern are correct
///
/// Takes &[`Regex`] to detect markers
/// // TODO any marker detection, currently only task-specified is implemented
///
/// # Errors
/// Returns [`Error::InvalidMarkersValue`] if a marker cannot be parsed into usize
///
/// Returns [`Error::TooBigMarkerValue`] if a marker was succesfully parsed
/// but is equals 0 or greater than marker_max
///
/// # Examples
///
/// ```
/// use regex::Regex;
/// use mmv::filename_operations::modified::check_markers_correctness;
/// let regex = Regex::new("#([0-9]+)").unwrap();
/// let pattern_good = "#1 #2 #3";
/// let pattern_bad = "#0 #5";
/// assert!(
///   check_markers_correctness(&regex, pattern_good, 3).is_ok()
/// );
/// assert!(
///   check_markers_correctness(&regex, pattern_bad, 3).is_err()
/// );
/// ```
pub fn check_markers_correctness(
    regex: &Regex,
    pattern: &str,
    marker_max: usize,
) -> Result<(), Error> {
    for digits in regex.find_iter(pattern) {
        match digits.as_str().strip_prefix("#").unwrap().parse::<usize>() {
            Err(_) => return Err(Error::InvalidMarkersValue),
            Ok(marker) if marker == 0 || marker > marker_max => {
                return Err(Error::TooBigMarkerValue);
            }
            Ok(_) => {}
        }
    }
    Ok(())
}

/// Modifies [`MatchedFilename`] into [`ModifiedFilename`] according to pattern
///
/// # Errors
///
/// All errors are handled in [`check_markers_correctness`] and then propagated higher
///
/// # Examples
/// ```
/// use std::ffi::OsString;
/// use mmv::filename_operations::{modified::modify_filename, matched::MatchedFilename};
/// let matched_name = MatchedFilename::new(OsString::from("filename"));
/// assert!(modify_filename(matched_name.clone(), "other_name").is_ok());
/// assert!(modify_filename(matched_name, "#0 new_name").is_err());
/// ```
pub fn modify_filename(matched: MatchedFilename, pattern: &str) -> Result<ModifiedFilename, Error> {
    let regex = Regex::new("#([0-9]+)").unwrap();
    let marker_max_value = matched.fragments.len();
    check_markers_correctness(&regex, pattern, marker_max_value)?;
    let changed_filename = Regex::replace_all(&regex, pattern, |digits: &Captures| -> &str {
        let marker = digits[0]
            .strip_prefix("#")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        &matched.fragments[marker - 1]
    });
    Ok(ModifiedFilename {
        origin: matched.filename,
        changed: changed_filename.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::filename_operations::matched::MatchedFilename;
    use crate::filename_operations::modified::{check_markers_correctness, modify_filename};
    use regex::Regex;
    use std::ffi::OsString;

    #[test]
    fn modify_filename_ok() {
        fn modify_filename_ok(
            filename: &str,
            fragments: Vec<&str>,
            pattern: &str,
            result_filename: &str,
        ) {
            let filename = OsString::from(filename.to_string());
            let modified = modify_filename(
                MatchedFilename {
                    filename: filename.clone(),
                    fragments: fragments
                        .into_iter()
                        .map(|fragment| fragment.to_string())
                        .collect(),
                },
                pattern,
            )
            .unwrap();
            assert_eq!(filename, modified.origin);
            assert_eq!(result_filename, modified.changed);
        }
        modify_filename_ok("abo_ba", vec!["abo", "ba"], "#2_#1", "ba_abo");
        modify_filename_ok(
            "money_pahnut_***",
            vec!["money", "pahnut", "***"],
            "#3_#2_#1",
            "***_pahnut_money",
        );
    }

    #[test]
    fn marker_correctness_checker() {
        let regex = Regex::new("#([0-9]+)").unwrap();
        let check = |pattern, marker_max| check_markers_correctness(&regex, pattern, marker_max);
        assert!(check("memi", 0).is_ok());
        assert!(check("#1 228", 1).is_ok());
        assert!(check("#aboba", 0).is_ok());
        assert!(check("#-123", 0).is_ok());
        assert!(check("#123", 1).is_err());
    }
}
