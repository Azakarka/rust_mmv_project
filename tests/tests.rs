use std::path::PathBuf;

use mmv::errors::CommonError as Error;
pub mod utils;
pub use utils::file_manager::FileManager;
use utils::{run::*, utils::*};

#[test]
fn ok_simple() -> Result<(), Error> {
    let manager = FileManager::from_test_name(TestName::OkSimple);
    /* let test_run = |
        filenames_to_match: Vec<&str>,
        modified_filenames: Vec<&str>,
        input_pattern: &str,
        output_pattern: &str
    | {
        match do_run(&manager.working_directory,
            filenames_to_match, modified_filenames,
            input_pattern, output_pattern) {
            Err(error) => {
                leave_test(TestName::OkSimple);
                panic!("{}", error);
            },
            Ok(_) => ()
        }
    }; */
    do_run(
        &manager,
        vec!["b_a", "d_c", "lolo_bubu"],
        vec!["a_b", "c_d", "bubu_lolo"],
        "*_*",
        "#2_#1",
    )?;
    do_run(
        &manager,
        vec!["привет_мир!", "прощай_мироздание!"],
        vec!["мир, привет!", "мироздание, прощай!"],
        "*_*!",
        "#2, #1!",
    )?;
    do_run(
        &manager,
        vec!["😀😃_😄😁"],
        vec!["😄😁_😀😃"],
        "*_*",
        "#2_#1",
    )?;
    Ok(())
}

#[test]
fn ok_special() -> Result<(), Error> {
    let manager = FileManager::from_test_name(TestName::OkSpecial);
    do_run(
        &manager,
        vec!["#1_#2_#3", "*_*_*", "!??_?!?_!??"],
        vec!["#3#1#2", "***", "!??!???!?"],
        "*_*_*",
        "#3#1#2",
    )?;
    Ok(())
}
#[test]
fn no_matched_files() {
    let manager = FileManager::from_test_name(TestName::NoMatchedFiles);
    do_run(&manager, vec!["a"], vec!["a"], "123", "#1")
        .expect_err("Files for pattern \"123\" not found");
}

fn force_flag_test(test_name: TestName) -> Result<(), Error> {
    let force = match test_name {
        TestName::FileExistsForceEnabled => true,
        TestName::FileExistsNoForce => false,
        _ => unreachable!(),
    };
    let manager = FileManager::from_test_name(test_name);
    let existing_filename_before = "foo.before";
    let existing_filename_after = "foo.after";
    manager.create_files(&vec![&existing_filename_after, &existing_filename_before]);
    check_run(RunParams {
        input_directory: manager.get_directory(),
        output_directory: manager.get_directory(),
        input_pattern: "*.before",
        output_pattern: "#1.after",
        filenames_to_match: vec![&existing_filename_before],
        modified_filenames: vec![&existing_filename_after],
        force: Some(force),
    })?;
    Ok(())
}

#[test]
fn file_exists_no_force() {
    force_flag_test(TestName::FileExistsNoForce).expect_err(&format!(
        "Not able to replace existing file \"do_not_replace.me\""
    ));
}

#[test]
fn file_exists_force_enabled() {
    assert!(force_flag_test(TestName::FileExistsForceEnabled).is_ok());
}

#[test]
fn different_directories() {
    let manager = FileManager::from_test_name(TestName::DifferentDirectories);
    let submanager_1 = manager.create_subdirectory(PathBuf::from("1/").as_path());
    let submanager_2 = manager.create_subdirectory(PathBuf::from("2/").as_path());
    let filenames = vec!["foo.txt", "bar.txt"];
    submanager_1.create_files(&filenames);
    assert!(check_run(RunParams {
        input_directory: &submanager_1.get_directory(),
        output_directory: &submanager_2.get_directory(),
        input_pattern: "*",
        output_pattern: "#1",
        filenames_to_match: filenames.clone(),
        modified_filenames: filenames,
        force: None
    })
    .is_ok());
}
