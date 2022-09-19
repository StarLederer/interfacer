use std::path::{Path, PathBuf};

use crate::tests::{messages::*, util};

use crate::api::*;
use crate::config::ActionConfig;
use crate::state::{ActionState, AppState};

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
    let to = util::fs::canonicalize("./src/tests/tmp/cloned-from-git");

    util::fs::rimraf(&to);

    todo!("Pull a git repo");
}

#[test]
fn git_detects_changes() {
    // Copy fixtures/git-app to tmp/git-app
    let from = util::fs::canonicalize("./src/tests/fixtures/git-app");
    let to = util::fs::canonicalize("./src/tests/tmp/git-app");

    util::fs::rimraf(&to);

    util::fs::copy_dir(from, to);

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

    let mut new_file_path = util::fs::canonicalize("./src/tests/tmp/git-app/workspace");
    new_file_path.push("new-file");

    util::fs::write(new_file_path, []);

    todo!("Check source control. Should detect new file");
}

#[test]
fn git_fetches() {
    util::fs::rimraf("./src/tests/tmp/git-repo");
    util::fs::rimraf("./src/tests/tmp/git-repo-2");

    // Clone test fixture
    // TODO: A local git server would be much better
    let mut to = util::fs::canonicalize("./src/tests/tmp");
    to.push("git-repo");
    let url = "https://github.com/StarLederer/git-test-fixture.git";
    let repo = match git2::Repository::clone(url, to) {
        Ok(repo) => repo,
        Err(err) => panic!(
            "{}",
            String::from("Failed to clone git-test-fixture!") + &err.to_string() + " " + TEST_ERR
        ),
    };

    // Copy the repo and open it
    util::fs::copy_dir("./src/tests/tmp/git-repo", "./src/tests/tmp/git-repo-2");
    let repo_2 = match git2::Repository::open("./src/tests/tmp/git-repo-2") {
        Ok(repo) => repo,
        Err(err) => panic!(
            "{}",
            String::from("Failed to clone git-test-fixture!") + &err.to_string() + " " + TEST_ERR
        ),
    };

    // Make a change to the copied repo and push
    util::fs::write("./src/tests/tmp/git-repo-2/nothing", []);
    let mut index = util::expect(repo_2.index(), "Failed to retrieve repo index");
    util::expect(
        index.add_path(Path::new("nothing")),
        "Failed to add file to index",
    );
    util::expect(index.write(), "Failed to write repo index to disk");
    util::expect(
        repo_2.commit(
            Some("HEAD"),
            &git2::Signature::now("Wrapp", "app@webwriter.org").unwrap(),
            &git2::Signature::now("Wrapp", "app@webwriter.org").unwrap(),
            "Update nothing",
            &repo_2.find_tree(repo_2.index().unwrap().write_tree().unwrap()).unwrap(),
            &[&repo_2.head().unwrap().peel_to_commit().unwrap()],
        ),
        "Failed to commit a change to git repo",
    );
    // repo_2.find_remote("origin").unwrap().push(refspecs, opts)

    // Fetch the now outdated repo

    // git_fetch(repo, "origin");
}

#[test]
fn git_saves_changes() {
    todo!("Check if git is able to pull, add changes, commit and push");
}
