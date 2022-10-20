use std::{str, path::Path};

pub fn clone(
    url: &str,
    into: &Path,
    username: &str,
    password: &str,
) -> Result<(), git2::Error> {
    // Remote callbacks
    let mut cb = git2::RemoteCallbacks::new();
    cb.credentials(move |_, _, _| git2::Cred::userpass_plaintext(username, password));

    // Fetch options
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);

    // Repo builder
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone
    builder.clone(url, into)?;

    Ok(())
}


pub fn fetch(
    repo: &git2::Repository,
    remote: &str,
    username: &str,
    password: &str,
) -> Result<bool, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();

    // Figure out whether it's a named remote or a URL
    let mut remote = repo
        .find_remote(remote)
        .or_else(|_| repo.remote_anonymous(remote))?;

    // Credentials
    cb.credentials(move |_, _, _| git2::Cred::userpass_plaintext(username, password));

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);

    // Fetch explicitly
    // remote.download(&[] as &[&str], Some(&mut fo))?;
    // remote.disconnect()?;
    // remote.update_tips(None, true, git2::AutotagOption::Unspecified, None)?;

    // Fetch conveniently
    remote.fetch(
        &[] as &[&str],
        Some(&mut fo),
        None,
    )?;

    let stats = remote.stats();
    stats.received_objects();

    Ok(stats.received_objects() > 0)
}

pub fn status(
    repo: &git2::Repository
) -> Result<bool, git2::Error> {
    let stati = repo.statuses(None)?;
    Ok(stati.len() > 0)
}

pub fn nuke_pull() -> Result<bool, git2::Error>  {
    todo!("fetch, merge, prefer local changes if any conflicts occur.");
}
