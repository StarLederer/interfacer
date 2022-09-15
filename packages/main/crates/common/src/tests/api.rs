use std::path::{Path, PathBuf};

use crate::api::*;
use crate::config::ActionConfig;
use crate::state::{ActionState, AppState};
use crate::tests::messages::*;

#[test]
fn self_terminated_interactions_work() {
    let mut action = ActionState {
        config: ActionConfig {
            command: String::from("ls -a"),
            user_terminated: false,
            idle_name: String::from("List"),
            active_name: String::from(""),
        },
        process: None,
    };

    interact(&mut action, Path::new("./")).unwrap();

    if action.process.is_some() {
        panic!(
            "The process field of the ActionState struct evaluated to Some when not user_terinated"
        );
    }
}

#[test]
fn user_terminated_interactions_work() {
    let mut action = ActionState {
        config: ActionConfig {
            command: String::from("top"),
            user_terminated: true,
            idle_name: String::from("Open bash"),
            active_name: String::from("Close bash"),
        },
        process: None,
    };

    // Start action
    interact(&mut action, Path::new("./")).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    if action.process.is_none() {
        panic!("The process field of the ActionState struct evaluated to None before user termination when user_terminated");
    }

    let pid = action.process.as_ref().unwrap().id();

    // Stop action
    interact(&mut action, Path::new("./")).unwrap();

    if String::from_utf8(
        std::process::Command::new("ps")
            .arg("-p")
            .arg(&pid.to_string())
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .contains(&pid.to_string())
    {
        // This seems to always fail. I suspect it is a NixOS thing
        panic!("ps found an active process with the same id as the child proess which means it was not terminated");
    }

    if action.process.is_some() {
        panic!("The process field of the ActionState struct evaluated to Some after user termination when user_terminated");
    }
}

#[test]
fn git_pulls_changes() {
    let to = std::fs::canonicalize("./src/tests/tmp/cloned-from-git")
        .expect(&(String::from(CANONICALIZE_ERR) + " " + TEST_ERR));

    match std::fs::remove_dir_all(to) {
        Ok(_) => {}
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {}
                _ => panic!(
                    "{}",
                    String::from("Failed to remove tmp/cloned-from-git!") + " " + TEST_ERR
                ),
            };
        }
    };

    todo!("Pull a git repo");
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
        Ok(_) => {}
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {}
                _ => panic!(
                    "{}",
                    String::from("Failed to remove tmp/git-app!") + " " + TEST_ERR
                ),
            };
        }
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
