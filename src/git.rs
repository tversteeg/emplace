use anyhow::{Context, Result};
use itertools::Itertools;
use log::{debug, error};
use std::{
    path::Path,
    process::{Command, Stdio},
};

fn call_on_path<P: AsRef<Path>>(command: Vec<&str>, path: &P) -> Result<bool> {
    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd.current_dir(path);

    debug!(
        "Calling \"{}\" on path {:?}",
        command.iter().join(" "),
        path.as_ref()
    );

    for arg in iter {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }

    let mut child = cmd.spawn().context("failed spawning process")?;
    let result = child.wait().context("failed waiting for result")?;

    Ok(result.success())
}

pub fn commit_all<P: AsRef<Path>>(dir: &P, msg: &str, sign: bool) -> Result<bool> {
    call_on_path(
        vec!["git", "commit", if sign { "-S" } else { "" }, "-am", msg],
        dir,
    )
    .context("failed commiting everything in git")
}

pub fn push<P: AsRef<Path>>(dir: &P) -> Result<bool> {
    call_on_path(vec!["git", "push"], dir).context("failed pushing in git")
}

/// Perform a pull on the git repo by fetching & merging.
pub fn pull<P: AsRef<Path>>(dir: &P, branch: &str) -> Result<bool> {
    // First fetch the remote changes
    call_on_path(
        vec![
            "git",
            "fetch",
            // We don't care about tags
            "--no-tags",
            // We don't care about submodules
            "--no-recurse-submodules",
            "origin",
            branch,
        ],
        dir,
    )
    .context("failed pulling in git: fetch")?;
    // Then merge them into this branch
    call_on_path(
        vec![
            "git",
            "merge",
            "--strategy-option",
            "theirs",
            "origin",
            branch,
        ],
        dir,
    )
    .context("failed pulling in git: merge")
}

pub fn clone_single_branch<P: AsRef<Path>>(dir: &P, url: &str, branch: &str) -> Result<bool> {
    let success = call_on_path(
        vec![
            "git",
            "clone",
            "--single-branch",
            "--branch",
            branch,
            url,
            ".",
        ],
        dir,
    )
    .context("failed cloning branch in git")?;

    if !success {
        // The git process reported that it succeeded but no clone has been done successfully
        error!("Cloning git branch failed, please execute the following code manually:\n\n\tgit clone --single-branch --branch {} {} {}\n", branch, url, dir.as_ref().to_str().unwrap())
    }

    Ok(success)
}

pub fn add_file<P: AsRef<Path>>(dir: &P, file: &str) -> Result<bool> {
    call_on_path(vec!["git", "add", file], dir).context("failed adding file in git")
}
