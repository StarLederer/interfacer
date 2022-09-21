use std::path::{Path, PathBuf};

use crate::git::*;
use crate::tests::{messages::*, util};

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

    let state = crate::state::AppState::init(
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
    util::init_env();
    util::fs::rimraf("./src/tests/tmp/git-repo");
    util::fs::rimraf("./src/tests/tmp/git-repo-2");
    let username = util::env::var("TEST_GIT_USERNAME");
    let password = util::env::var("TEST_GIT_PASSWORD");
    let fetch_fail_msg = "Failed to fetch from a git repo!";

    // Clone test fixture
    // TODO: A local git server would be much better
    let mut to = util::fs::canonicalize("./src/tests/tmp");
    to.push("git-repo");
    let url = util::env::var("TEST_GIT_REPO");
    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.remote_callbacks(util::git2::RemoteCallbacks::new());
    let mut repo_builder = git2::build::RepoBuilder::new();
    repo_builder.fetch_options(fetch_opts);
    let repo = match repo_builder.clone(&url, &to) {
        Ok(repo) => repo,
        Err(err) => panic!(
            "{}",
            String::from("Failed to clone git-test-fixture!") + &err.to_string() + " " + TEST_ERR
        ),
    };

    // Fetch the repo, should detect no changes
    assert_eq!(
        git_fetch(&repo, "origin", &username, &password,).expect(&fetch_fail_msg),
        false
    );

    // Copy the repo and open it
    util::fs::copy_dir("./src/tests/tmp/git-repo", "./src/tests/tmp/git-repo-2");
    let repo_2 = match git2::Repository::open("./src/tests/tmp/git-repo-2") {
        Ok(repo) => repo,
        Err(err) => panic!(
            "{}",
            String::from("Failed to clone git-test-fixture!") + &err.to_string() + " " + TEST_ERR
        ),
    };

    // Make a change to the copied repo
    util::fs::write(
        "./src/tests/tmp/git-repo-2/time",
        util::expect(
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH),
            "Error retreiving time. System time cannot be set to before the unix epoch",
        )
        .as_millis()
        .to_be_bytes(),
    );
    let mut index = util::expect(repo_2.index(), "Failed to retrieve repo index");
    util::expect(
        index.add_path(Path::new("time")),
        "Failed to add file to index",
    );
    util::expect(index.write(), "Failed to write repo index to disk");
    util::expect(
        repo_2.commit(
            Some("HEAD"),
            &git2::Signature::now("Wrapp", "app@webwriter.org").unwrap(),
            &git2::Signature::now("Wrapp", "app@webwriter.org").unwrap(),
            "Update time",
            &repo_2
                .find_tree(repo_2.index().unwrap().write_tree().unwrap())
                .unwrap(),
            &[&repo_2.head().unwrap().peel_to_commit().unwrap()],
        ),
        "Failed to commit a change to git repo",
    );

    // Push changes to the copied repo
    let mut origin = util::expect(
        repo_2.find_remote("origin"),
        "Failed to find remote \"origin\"",
    );
    origin
        .push(
            &["HEAD:refs/heads/main"],
            Some(git2::PushOptions::new().remote_callbacks(util::git2::RemoteCallbacks::new())),
        )
        .expect("Failed to push to git remote");

    // Fetch the now outdated repo, should detech changes
    assert_eq!(
        git_fetch(&repo, "origin", &username, &password,).expect(&fetch_fail_msg),
        true
    );
}

#[test]
fn git_saves_changes() {
    todo!("Check if git is able to pull, add changes, commit and push");
}
