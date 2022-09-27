use std::str;

pub fn git_fetch(
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
