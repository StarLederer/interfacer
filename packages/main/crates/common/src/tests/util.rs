#![allow(unused)]

use dotenv::dotenv;
use std::sync::Once;

use super::messages::*;

static INIT: Once = Once::new();

/**
 * Initializes the environment
 */
pub fn init_env() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}

/**
 * Like unwrap but panics with a custom error message
 */
pub fn unwrap<T>(option: Option<T>) -> T {
    match option {
        Some(s) => s,
        None => {
            panic!(
                "{}",
                String::from("Failed to unwrap an Option!") + " " + TEST_ERR
            )
        }
    }
}

/**
 * Like expect but decorates the error message with super::messages::TEST_ERR
 */
pub fn expect<T, E>(option: Result<T, E>, message: &str) -> T
where
    E: std::fmt::Display,
{
    match option {
        Ok(o) => o,
        Err(e) => {
            panic!(
                "{}",
                String::from(message) + &e.to_string() + " " + TEST_ERR
            )
        }
    }
}

pub mod fs {
    use crate::tests::messages::*;

    pub fn canonicalize<P>(path: P) -> std::path::PathBuf
    where
        P: AsRef<std::path::Path>,
    {
        std::fs::canonicalize(path).expect(&(String::from(CANONICALIZE_ERR) + " " + TEST_ERR))
    }

    /**
     * Similar to rm -rf
     * Does not panic if the path does not exist
     */
    pub fn rimraf<P>(path: P)
    where
        P: AsRef<std::path::Path>,
    {
        match std::fs::remove_dir_all(path) {
            Ok(_) => {}
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => panic!(
                        "{}",
                        String::from("Failed to remove a directory!") + " " + TEST_ERR
                    ),
                };
            }
        };
    }

    pub fn copy_dir<P>(copy_dir: P, copy_to: P)
    where
        P: AsRef<std::path::Path>,
    {
        fs_extra::dir::copy(
            copy_dir,
            copy_to,
            &fs_extra::dir::CopyOptions {
                copy_inside: true,
                ..Default::default()
            },
        )
        .expect(&(String::from("Failed to copy directory!") + " " + TEST_ERR));
    }

    pub fn write<P, C>(path: P, contents: C)
    where
        P: AsRef<std::path::Path>,
        C: AsRef<[u8]>,
    {
        std::fs::write(path, contents)
            .expect(&(String::from("Failed to create a new file!") + " " + TEST_ERR));
    }

    pub fn create_dir_all<P: AsRef<std::path::Path>>(path: P) {
        std::fs::create_dir_all(path)
            .expect(&(String::from("Failed to create a directory!") + " " + TEST_ERR));
    }
}

pub mod env {
    use crate::tests::messages::*;

    pub fn var<K>(key: K) -> String
    where
        K: AsRef<std::ffi::OsStr>,
    {
        match std::env::var(key) {
            Ok(o) => o,
            Err(e) => {
                panic!(
                    "{}",
                    String::from("Failed to get environment variable!")
                        + " "
                        + &e.to_string()
                        + " "
                        + TEST_ERR
                )
            }
        }
    }
}

pub mod git2 {
    use crate::tests::messages::*;
    use std::path::Path;

    pub struct RemoteCallbacks;

    impl RemoteCallbacks {
        pub fn new<'a>() -> git2::RemoteCallbacks<'a> {
            let mut remote_callbacks = git2::RemoteCallbacks::new();

            remote_callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                git2::Cred::userpass_plaintext(
                    &super::env::var("TEST_GIT_USERNAME"),
                    &super::env::var("TEST_GIT_PASSWORD"),
                )
            });

            remote_callbacks
        }
    }

    pub fn clone_repo(url: &str, into: &Path) -> git2::Repository {
        let mut fetch_opts = git2::FetchOptions::new();
        fetch_opts.remote_callbacks(RemoteCallbacks::new());
        let mut repo_builder = git2::build::RepoBuilder::new();
        repo_builder.fetch_options(fetch_opts);

        match repo_builder.clone(url, into) {
            Ok(repo) => repo,
            Err(err) => panic!(
                "{}",
                String::from("Failed to clone git-test-fixture!")
                    + &err.to_string()
                    + " "
                    + TEST_ERR
            ),
        }
    }

    pub fn find_tree<'a>(repo: &'a git2::Repository, spec: &'a str) -> git2::Object<'a> {
        repo.revparse_single(spec)
            .expect(&(String::from("Could not find object with spec ") + spec + "! " + TEST_ERR))
            .peel(git2::ObjectType::Tree)
            .expect(&(String::from("Could not peel git object to tree!") + " " + TEST_ERR))
    }
}
