use anyhow::{Context, Result};
use log::error;
use std::{
    path::Path,
    process::{Command, Stdio},
};

fn call_on_path<P: AsRef<Path>>(command: Vec<&str>, path: &P, dry_run: bool) -> Result<bool> {
    if dry_run {
        println!("cd {}", path.as_ref().display());
        println!("{}", command.join(" "));
        return Ok(true);
    }

    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd.current_dir(path);

    for arg in iter {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }

    let mut child = cmd.spawn().context("failed spawning process")?;
    let result = child.wait().context("failed waiting for result")?;

    Ok(result.success())
}

pub fn commit_all<P: AsRef<Path>>(dir: &P, msg: &str, sign: bool, dry_run: bool) -> Result<bool> {
    call_on_path(
        vec!["git", "commit", if sign { "-S" } else { "" }, "-am", msg],
        dir,
        dry_run,
    )
    .context("failed commiting everything in git")
}

pub fn push<P: AsRef<Path>>(dir: &P, dry_run: bool) -> Result<bool> {
    call_on_path(vec!["git", "push"], dir, dry_run).context("failed pushing in git")
}

pub fn pull<P: AsRef<Path>>(dir: &P, dry_run: bool) -> Result<bool> {
    call_on_path(vec!["git", "pull"], dir, dry_run).context("failed pulling in git")
}

pub fn clone_single_branch<P: AsRef<Path>>(
    dir: &P,
    url: &str,
    branch: &str,
    dry_run: bool,
) -> Result<bool> {
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
        dry_run,
    )
    .context("failed cloning branch in git")?;

    if !success {
        // The git process reported that it succeeded but no clone has been done successfully
        error!("Cloning git branch failed, please execute the following code manually:\n\n\tgit clone --single-branch --branch {} {} {}\n", branch, url, dir.as_ref().to_str().unwrap())
    }

    Ok(success)
}

pub fn add_file<P: AsRef<Path>>(dir: &P, file: &str, dry_run: bool) -> Result<bool> {
    call_on_path(vec!["git", "add", file], dir, dry_run).context("failed adding file in git")
}
