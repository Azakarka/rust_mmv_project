//! Utils to [`match_filenames`] and [`modify_filenames`]
use super::matched::{match_filename, MatchedFilenames};
use crate::errors::CommonError as Error;
use crate::filename_operations::modified::{modify_filename, ModifiedFilenames};
use std::ffi::OsString;

/// Modifies [`MatchedFilenames`] according to pattern
///
/// Iterates over [`MatchedFilenames`] and converts them into [`ModifiedFilenames`]
/// using [`modify_filename`] function
///
/// # Errors
/// Propagates errors from [`modify_filename`]
pub fn modify_filenames(
    matched: MatchedFilenames,
    pattern: &str,
) -> Result<ModifiedFilenames, Error> {
    let mut modified: ModifiedFilenames = vec![];
    for matched_filename in matched {
        let modified_value = modify_filename(matched_filename, &pattern)?;
        modified.push(modified_value);
    }
    Ok(modified)
}

/// Mathches filenames according to pattern
///
/// Iterates over filenames and converts them into [`MatchedFilenames`]
/// using [`match_filename`] function
///
/// # Errors
/// Returns [`Error::NoMatchingFiles`] if resulting [`MatchedFilenames`] is empty
pub fn match_filenames(filenames: Vec<OsString>, pattern: &str) -> Result<MatchedFilenames, Error> {
    let mut matched: MatchedFilenames = vec![];
    for filename in filenames {
        if let Some(matched_value) = match_filename(&filename, &pattern) {
            matched.push(matched_value)
        }
    }
    if matched.is_empty() {
        Err(Error::NoMatchingFiles {
            pattern: pattern.to_string(),
        })
    } else {
        Ok(matched)
    }
}

/// Combines [`match_filenames`] and [`modify_filenames`] functions into one
///
/// # Errors
/// Propagates errors from according functions
pub fn match_and_modify_filenames(
    filenames: Vec<OsString>,
    input_pattern: &str,
    output_pattern: &str,
) -> Result<ModifiedFilenames, Error> {
    let matched_filenames = match_filenames(filenames, input_pattern)?;
    Ok(modify_filenames(matched_filenames, output_pattern)?)
}

#[cfg(test)]
mod tests {
    use crate::filename_operations::operations::match_and_modify_filenames;
    use std::ffi::OsString;

    #[test]
    fn match_and_modify_ok() {
        fn match_and_modify_ok(
            filename: &str,
            input_pattern: &str,
            output_pattern: &str,
            expected: &str,
        ) {
            let filename = OsString::from(filename.to_string());
            assert_eq!(
                match_and_modify_filenames(vec![filename], input_pattern, output_pattern).unwrap()
                    [0]
                .changed,
                expected
            );
        }

        match_and_modify_ok("sus_amogus", "*_*", "#2_#1", "amogus_sus");
        match_and_modify_ok("biba_boba_buba", "*_*_*", "#3-#1-#2", "buba-biba-boba");
        match_and_modify_ok("bruh", "*", "#1", "bruh")
    }
}
