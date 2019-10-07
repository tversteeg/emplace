use std::{
    error::Error,
    path::Path,
    process::{Command, Stdio},
};

fn call_on_path<P: AsRef<Path>>(
    command: Vec<&str>,
    path: &P,
    dry_run: bool,
) -> Result<bool, Box<dyn Error>> {
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

    let mut child = cmd.spawn()?;
    let result = child.wait()?;

    Ok(result.success())
}

pub fn commit_all<P: AsRef<Path>>(
    dir: &P,
    msg: &str,
    sign: bool,
    dry_run: bool,
) -> Result<bool, Box<dyn Error>> {
    call_on_path(
        vec!["git", "commit", if sign { "-S" } else { "" }, "-am", msg],
        dir,
        dry_run,
    )
}

pub fn push<P: AsRef<Path>>(dir: &P, dry_run: bool) -> Result<bool, Box<dyn Error>> {
    call_on_path(vec!["git", "push"], dir, dry_run)
}

pub fn pull<P: AsRef<Path>>(dir: &P, dry_run: bool) -> Result<bool, Box<dyn Error>> {
    call_on_path(vec!["git", "pull"], dir, dry_run)
}

pub fn clone_single_branch<P: AsRef<Path>>(
    dir: &P,
    url: &str,
    branch: &str,
    dry_run: bool,
) -> Result<bool, Box<dyn Error>> {
    call_on_path(
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
}

pub fn add_file<P: AsRef<Path>>(
    dir: &P,
    file: &str,
    dry_run: bool,
) -> Result<bool, Box<dyn Error>> {
    call_on_path(vec!["git", "add", file], dir, dry_run)
}
