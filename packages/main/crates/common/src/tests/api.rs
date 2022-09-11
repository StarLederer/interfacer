use std::path::Path;

use crate::api::*;
use crate::config::ActionConfig;
use crate::state::ActionState;

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
