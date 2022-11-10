use std::{path::Path, str};

pub fn clone(url: &str, into: &Path, username: &str, password: &str) -> Result<(), git2::Error> {
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
    // Figure out whether it's a named remote or a URL
    let mut remote = repo
        .find_remote(remote)
        .or_else(|_| repo.remote_anonymous(remote))?;

    // Remote callbacks
    let mut cb = git2::RemoteCallbacks::new();
    cb.credentials(move |_, _, _| git2::Cred::userpass_plaintext(username, password));

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);

    // Fetch explicitly
    // remote.download(&[] as &[&str], Some(&mut fo))?;
    // remote.disconnect()?;
    // remote.update_tips(None, true, git2::AutotagOption::Unspecified, None)?;

    // Fetch conveniently
    remote.fetch(&[] as &[&str], Some(&mut fo), None)?;

    let stats = remote.stats();
    stats.received_objects();

    Ok(stats.received_objects() > 0)
}

pub fn status(repo: &git2::Repository) -> Result<bool, git2::Error> {
    let stati = repo.statuses(None)?;
    Ok(stati.len() > 0)
}

/**
 * Pulls changes and prefers remote versions of all conflicts
 *
 * # TODO
 * - Implement ability to manually select changes
 * - Rename to just "pull" when previous point is complete
 */
pub fn nuke_pull(
    repo: &git2::Repository,
    remote: &str,
    username: &str,
    password: &str,
) -> Result<(), git2::Error> {
    // Fetch
    fetch(repo, remote, username, password)?;

    // Merge (prefer remote changes)
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    // Discard all local changes ("nuke")
    repo.reset(
        repo.head()?.peel_to_commit()?.as_object(),
        git2::ResetType::Hard,
        None,
    )?;

    // Merge (prefer remote changes (should not occur since we discard before merging))
    let mut merge_opts = git2::MergeOptions::new();
    merge_opts.file_favor(git2::FileFavor::Theirs);
    let mut checkout_opts = git2::build::CheckoutBuilder::new();
    checkout_opts.force();
    checkout_opts.use_theirs(true);
    repo.merge(
        &[&fetch_commit],
        Some(&mut merge_opts),
        Some(&mut checkout_opts),
    )?;

    commit(repo)?;

    repo.cleanup_state()?;

    Ok(())
}

pub fn add_all(repo: &git2::Repository) -> Result<(), git2::Error> {
    // Stage changes
    let mut index = repo.index()?;
    // We need CHECK_PATHSPEC to emulate git add -A
    // without it we emilate git add . which does not stage deletions
    index.add_all(
        ["."].iter(),
        git2::IndexAddOption::DEFAULT.union(git2::IndexAddOption::CHECK_PATHSPEC),
        None,
    )?; // not sure if we need to union with DEFAULT
    index.write()?;

    Ok(())
}

/**
 * Commit current index with the preset message
 */
pub fn commit(repo: &git2::Repository) -> Result<git2::Oid, git2::Error> {
    repo.commit(
        Some("HEAD"),
        &git2::Signature::now("Interfacer", "app@interfacer.org").unwrap(),
        &git2::Signature::now("Interfacer", "app@interfacer.org").unwrap(),
        "Save changes using Interfacer",
        &repo.find_tree(repo.index()?.write_tree()?)?,
        &[&repo.head()?.peel_to_commit()?],
    )
}

pub fn push(
    repo: &git2::Repository,
    remote: &str,
    username: &str,
    password: &str,
) -> Result<(), git2::Error> {
    // Get remote by given name
    let mut remote = repo.find_remote(remote)?;

    // Remote callbacks
    let mut cb = git2::RemoteCallbacks::new();
    cb.credentials(move |_, _, _| git2::Cred::userpass_plaintext(username, password));

    // Push options
    let mut po = git2::PushOptions::new();
    po.remote_callbacks(cb);

    // Iterate over each branch
    for branch in repo.branches(None)? {
        let branch = branch?;

        // Continue on every branch that is local and HEAD.
        // Normally there is only one HEAD in a Git repo
        if branch.0.is_head() && branch.1 == git2::BranchType::Local {
            let branch_name = match branch.0.name()? {
                Some(name) => name,
                None => return Err(git2::Error::new(git2::ErrorCode::Invalid, git2::ErrorClass::Repository, "Current branch name is not a valid utf-8 string")),
            };

            // Push
            remote.push(&[format!("HEAD:refs/heads/{}", branch_name)], Some(&mut po))?;
        }
    }

    Ok(())
}
