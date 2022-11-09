use std::path::{Path, PathBuf};

#[allow(unused_imports)]
use crate::tests::{messages::*, util};

use crate::{git::*};


/// # Assumptions
/// * "origin" is the remote that we want to push to
/// * we want to push changes on HEAD to main
fn push_repo(repo: &git2::Repository) {
    let mut origin = util::expect(
        repo.find_remote("origin"),
        "Failed to find remote \"origin\"",
    );
    origin
        .push(
            &["HEAD:refs/heads/main"],
            Some(git2::PushOptions::new().remote_callbacks(util::git2::RemoteCallbacks::new())),
        )
        .expect("Failed to push to git remote");
}

fn write_time_to_repo(repo: &git2::Repository, custom_time: Option<u128>) {
    // Find workdir
    let workdir = util::expect_opt(repo.workdir(), "Repo doesn't have a workdir!");

    // Make a change to worktree
    let mut file_path = PathBuf::from(workdir);
    file_path.push("time");
    let time = util::expect(
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH),
        "Error retreiving time. System time cannot be set to before the unix epoch",
    );
    util::fs::write(file_path, custom_time.unwrap_or(time.as_millis()).to_be_bytes());

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
    util::init_env();

    let username = util::env::var("TEST_GIT_USERNAME");
    let password = util::env::var("TEST_GIT_PASSWORD");
    let mut repo1_path = util::fs::tmp_path();
    repo1_path.push("repo1");
    let mut repo2_path = util::fs::tmp_path();
    repo2_path.push("repo2");

    // Rimraf paths we need for testing
    util::fs::rimraf(&repo1_path);
    util::fs::rimraf(&repo2_path);

    // Clone repo as repo 1 and open it
    let url = util::env::var("TEST_GIT_REPO");
    let repo1 = util::git2::clone_repo(&url, &repo1_path);

    // Copy repo 1 to repo 2 and open it
    util::fs::copy_dir(&repo1_path, &repo2_path);
    let repo2 = util::expect(
        git2::Repository::open(&repo2_path),
        "Could not open a git repo!",
    );

    // Test macro
    macro_rules! assert_repos_match {
        () => {
            // Time in repo 1 and repo 2 should have matching contents
            let mut repo1_time_path = repo1_path.clone();
            let mut repo2_time_path = repo2_path.clone();
            repo1_time_path.push("time");
            repo2_time_path.push("time");
            let repo1_time = util::fs::read(repo1_time_path);
            let repo2_time = util::fs::read(repo2_time_path);
            assert_eq!(repo1_time, repo2_time);
        };
    }

    // Test without conflicts
    {
        // Advance repo 2
        write_time_to_repo(&repo2, None);
        push_repo(&repo2);

        // Test
        nuke_pull(&repo1, "origin", &username, &password).expect("Failed to pull remote branch");
        assert_repos_match!();
    }

    // Test with conflicts
    {
        // Make a local change to repo 1
        let mut file_path = PathBuf::from(&repo1_path);
        file_path.push("time");
        util::fs::write(file_path, "this should cause a conflict");

        // Advance repo 2
        write_time_to_repo(&repo2, None);
        push_repo(&repo2);

        // Test
        nuke_pull(&repo1, "origin", &username, &password)
            .expect("Failed to nukepull remote branch with merge conflicts");
        assert_repos_match!();
    }
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
    assert_eq!(status(&repo).expect(status_fail_msg), false);

    // Make a change to worktree
    let mut file_path = PathBuf::from(repo_path);
    file_path.push("newfile");
    util::fs::write(file_path, []);

    // Check repo_2. Should detect changes now
    assert_eq!(status(&repo).expect(status_fail_msg), true);
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
    write_time_to_repo(&repo_2, None);

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
    util::init_env();

    let username = util::env::var("TEST_GIT_USERNAME");
    let password = util::env::var("TEST_GIT_PASSWORD");
    let url = util::env::var("TEST_GIT_REPO");
    let repo1_path = Path::new("./src/tests/tmp/git_saves_changes_repo1");
    let repo2_path = Path::new("./src/tests/tmp/git_saves_changes_repo2");

    // Prepare the test repo
    util::fs::rimraf(&repo1_path);
    let repo = util::git2::clone_repo(&url, &repo1_path);

    // Test saving with no conflicts
    {
        // Make a change
        write_time_to_repo(&repo, None);

        // Save
        add_all(&repo).expect("Failed to stage changes");
        commit(&repo).expect("Faield to commit changes");
        push(&repo, "origin", &username, &password).expect("Failed to push repo");
    }

    // Test with conflicts
    {
        // Introduce a conflict
        {
            // Prepare repo 2 path
            util::fs::rimraf(&repo2_path);

            // Copy repo 1 to repo 2 and open it
            util::fs::copy_dir(&repo1_path, &repo2_path);
            let repo2 = util::expect(
                git2::Repository::open(&repo2_path),
                "Could not open a git repo!",
            );

            // Write 0 as time
            write_time_to_repo(&repo2, Some(0));
            push_repo(&repo2);
        }

        // Advance repo 1
        write_time_to_repo(&repo, None);

        // Save
        add_all(&repo).expect("Failed to stage changes");
        commit(&repo).expect("Faield to commit changes");
        match push(&repo, "origin", &username, &password) {
            Ok(_) => panic!("Successfully pushed but expected to fail with a conflict"),
            Err(err) => match err.code() {
                git2::ErrorCode::NotFastForward => {
                    // We want this. This is correct
                    // This is the error that indicates that testrepo's main on the remote has commits that local main does not.
                    // That is the commit that we push just above
                },
                _ => {
                    // Some other error occured while pushing
                    // This most likely indicates a test error but
                    // I am not sure
                    panic!("An error occurred when pushing. It is likely (but not certainly) a test error. Please inspect the cause. ({})", err);

                },
            },
        }
    }
}
