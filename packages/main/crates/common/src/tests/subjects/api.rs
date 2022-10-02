use std::path::{Path, PathBuf};

use crate::tests::{messages::*, util};

use crate::api::internal::*;
use crate::project_config::ActionConfig;

#[test]
fn self_terminated_interactions_work() {
    let mut action = crate::state::ActionState {
        config: ActionConfig {
            command: String::from("ls -a"),
            user_terminated: false,
            idle_name: String::from("List"),
            active_name: String::from(""),
        },
        process: None,
    };

    interact_directly(&mut action, Path::new("./")).unwrap();

    if action.process.is_some() {
        panic!(
            "The process field of the ActionState struct evaluated to Some when not user_terinated"
        );
    }
}

#[test]
fn user_terminated_interactions_work() {
    let mut action = crate::state::ActionState {
        config: ActionConfig {
            command: String::from("top"),
            user_terminated: true,
            idle_name: String::from("Open bash"),
            active_name: String::from("Close bash"),
        },
        process: None,
    };

    // Start action
    interact_directly(&mut action, Path::new("./")).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    if action.process.is_none() {
        panic!("The process field of the ActionState struct evaluated to None before user termination when user_terminated");
    }

    let pid = action.process.as_ref().unwrap().id();

    // Stop action
    interact_directly(&mut action, Path::new("./")).unwrap();

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
fn source_control_detects_changes() {
    // Copy fixtures/git-app to tmp/git-app
    let from = util::fs::canonicalize("./src/tests/fixtures/git-app");
    let to = util::fs::canonicalize("./src/tests/tmp/git-app");

    util::fs::rimraf(&to);

    util::fs::copy_dir(from, to);

    let state = crate::state::ProjectState::init(
        crate::project_config::Config {
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

    let mut new_file_path = util::fs::canonicalize("./src/tests/tmp/git-app/workspace");
    new_file_path.push("new-file");

    util::fs::write(new_file_path, []);

    todo!("Check source control. Should detect new file");
}
