use std::path::PathBuf;

use crate::state::*;

// Error messages
const TEST_ERR: &str = "THIS IS A TEST ERROR!";
#[allow(dead_code)]
const GENERIC_ERR: &str = "An error without description occured!";
const CANONICALIZE_ERR: &str = "Failed to canonicalize a path!";

#[test]
fn state_initializes() {
    let state = AppState::init(
        crate::config::Config {
            version: String::from("1"),
            workspace_dir: String::from("./workspace"),
            after_code_download: vec![],
            before_code_upload: vec![],
            actions: vec![],
        },
        PathBuf::from("./src/tests/fixtures/minimum-config-app"),
    );

    state.expect("Failed to init state from config");
}

#[test]
fn git_detects_changes() {
    // Copy fixtures/git-app to tmp/git-app
    let from = std::fs::canonicalize("./src/tests/fixtures/git-app")
        .expect(&(String::from(CANONICALIZE_ERR) + " " + TEST_ERR));
    let to_parent = std::fs::canonicalize("./src/tests/tmp")
        .expect(&(String::from(CANONICALIZE_ERR) + " " + TEST_ERR));

    let mut to = to_parent.clone();
    to.push("git-app");
    match std::fs::remove_dir_all(to) {
        Ok(_) => {},
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {},
                _ => panic!("{}", String::from("Failed to remove tmp/git-app!") + " " + TEST_ERR),
            };
        },
    };

    fs_extra::dir::copy(from, to_parent, &fs_extra::dir::CopyOptions::default())
        .expect(&(String::from("Failed to copy a fixture!") + " " + TEST_ERR));

    let state = AppState::init(
        crate::config::Config {
            version: String::from("1"),
            workspace_dir: String::from("./workspace"),
            after_code_download: vec![],
            before_code_upload: vec![],
            actions: vec![],
        },
        PathBuf::from("./src/tests/tmp/git-app"),
    )
    .expect(&(String::from("Failed to init state from config!") + " " + TEST_ERR));

    todo!("Check source control. Should detect no changes");

    let mut new_file_path = std::fs::canonicalize("./src/tests/tmp/git-app/workspace")
        .expect(&(String::from(CANONICALIZE_ERR) + " " + TEST_ERR));
    new_file_path.push("new-file");

    std::fs::write(new_file_path, [])
        .expect(&(String::from("Failed to create a new file!") + " " + TEST_ERR));

    todo!("Check source control. Should detect new file");
}

#[test]
fn git_saves_changes() {
    todo!("Check if git is able to pull, add changes, commit and push");
}
