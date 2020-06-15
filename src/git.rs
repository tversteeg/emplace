use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use log::{debug, error};
use std::{
    path::Path,
    process::{Command, Stdio},
    str,
};

fn build_command<P: AsRef<Path>>(command: Vec<&str>, path: &P) -> Result<Command> {
    let mut iter = command.iter();
    let cmd_name = iter
        .next()
        .ok_or_else(|| anyhow!("Malformed git command"))?;

    let mut cmd = Command::new(cmd_name);
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

    Ok(cmd)
}

fn call_on_path<P: AsRef<Path>>(command: Vec<&str>, path: &P) -> Result<bool> {
    let mut cmd = build_command(command, path)?;
    cmd.stdout(Stdio::null());

    let mut child = cmd.spawn().context("failed spawning process")?;
    let result = child.wait().context("failed waiting for result")?;

    Ok(result.success())
}

fn call_on_path_has_output<P: AsRef<Path>>(command: Vec<&str>, path: &P) -> Result<bool> {
    let mut cmd = build_command(command, path)?;

    Ok(str::from_utf8(&cmd.output()?.stdout)?.trim() != "")
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

pub fn clone_full(dir: &str, url: &str) -> Result<bool> {
    let dummy_path = std::path::PathBuf::from("./");
    call_on_path(vec!["git", "clone", url, dir], &dummy_path)
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
/// Set remote origin
pub fn set_remote<P: AsRef<Path>>(dir: &P, remote: &str) -> Result<bool> {
    call_on_path(vec!["git", "remote", "add", "origin", remote], dir)
        .context("failed setting remote origin")
}

/// Stage a specific file for commiting.
pub fn add_file<P: AsRef<Path>>(dir: &P, file: &str) -> Result<bool> {
    call_on_path(vec!["git", "add", file], dir).context("failed adding file in git")
}

/// Stage all files for commiting.
pub fn add_all_files<P: AsRef<Path>>(dir: &P) -> Result<bool> {
    call_on_path(vec!["git", "add", "-A"], dir).context("failed adding all files in git")
}

/// Do a git status to verify if there are local changes.
pub fn has_changes<P: AsRef<Path>>(dir: &P) -> Result<bool> {
    Ok(!call_on_path_has_output(
        vec!["git", "ls-files", "--others", "--exclude-standard"],
        dir,
    )
    .context("failed checking if there are git changes")?)
}

pub fn init_repo<P: AsRef<Path>>(dir: &P) -> Result<bool> {
    Ok(call_on_path(vec!["git", "init"], dir).context("Failed initializing the repository")?)
}
