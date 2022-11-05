use std::path::{Path, PathBuf};

#[allow(unused_imports)]
use crate::tests::{messages::*, util};

use crate::git::*;

fn write_time_to_repo(repo: &git2::Repository) {
    // Find workdir
    let workdir = util::expect_opt(repo.workdir(), "Repo doesn't have a workdir!");

    // Make a change to worktree
    let mut file_path = PathBuf::from(workdir);
    file_path.push("time");
    util::fs::write(
        file_path,
        util::expect(
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH),
            "Error retreiving time. System time cannot be set to before the unix epoch",
        )
        .as_millis()
        .to_be_bytes(),
    );

    // Stage the cahange
    let mut index = util::expect(repo.index(), "Failed to retrieve repo index");
    util::expect(
        index.add_path(Path::new("time")),
        "Failed to add file to index",
    );
    util::expect(index.write(), "Failed to write repo index to disk");
    util::expect(
        repo.commit(
            Some("HEAD"),
            &git2::Signature::now("Wrapp", "app@webwriter.org").unwrap(),
            &git2::Signature::now("Wrapp", "app@webwriter.org").unwrap(),
            "Update time",
            &repo
                .find_tree(repo.index().unwrap().write_tree().unwrap())
                .unwrap(),
            &[&repo.head().unwrap().peel_to_commit().unwrap()],
        ),
        "Failed to commit a change to git repo",
    );
}

#[test]
fn git_clones() {
    util::init_env();
    let url = util::env::var("TEST_GIT_REPO");
    let username = util::env::var("TEST_GIT_USERNAME");
    let password = util::env::var("TEST_GIT_PASSWORD");
    let into = Path::new("./src/tests/tmp/git-repo");

    util::fs::rimraf(&into);

    // Fetch the repo, should detect no changes
    clone(&url, into, &username, &password).unwrap();
}

#[test]
fn git_pulls_changes() {
    todo!("Pull a git repo");
}

#[test]
fn git_detects_changes() {
    util::init_env();

    let status_fail_msg = "Failed to status a git repo!";
    let mut repo_path = util::fs::canonicalize("./src/tests/tmp");
    repo_path.push("git-repo");

    util::fs::rimraf(&repo_path);

    // Clone test repo
    let url = util::env::var("TEST_GIT_REPO");
    let repo = util::git2::clone_repo(&url, &repo_path);

    // Check the repo. Should not detect cahnges yet
    assert_eq!(
        status(&repo).expect(status_fail_msg),
        false
    );

    // Make a change to worktree
    let mut file_path = PathBuf::from(repo_path);
    file_path.push("newfile");
    util::fs::write(file_path, []);

    // Check repo_2. Should detect changes now
    assert_eq!(
        status(&repo).expect(status_fail_msg),
        true
    );
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
    let repo = util::git2::clone_repo(&url, &to);

    // Fetch the repo, should detect no changes
    assert_eq!(
        fetch(&repo, "origin", &username, &password,).expect(&fetch_fail_msg),
        false
    );

    // Copy the repo and open it
    util::fs::copy_dir("./src/tests/tmp/git-repo", "./src/tests/tmp/git-repo-2");
    let repo_2 = util::expect(
        git2::Repository::open("./src/tests/tmp/git-repo-2"),
        "Could not open a git repo!",
    );

    // Make a change to the copied repo
    write_time_to_repo(&repo_2);

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
        fetch(&repo, "origin", &username, &password,).expect(&fetch_fail_msg),
        true
    );
}

#[test]
fn git_saves_changes() {
    todo!("Check if git is able to pull, add changes, commit and push");
}
