use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use log::debug;
use std::process::{Command, Stdio};

/// Perform a command.
pub fn call(command: &str, arguments: &Vec<&str>) -> Result<()> {
    debug!(
        "Performing command \"{}\" with arguments {}",
        command,
        arguments.iter().join(" ")
    );

    let mut cmd = Command::new(command);
    cmd.args(arguments);

    // Execute the command
    let result = cmd.output().context(format!(
        "executing command \"{}\" {}",
        command,
        arguments.iter().join(" ")
    ))?;

    // Return stderr as an error result when the command failed
    if result.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8(result.stderr)?;

        Err(anyhow!("{}", stderr))
    }
}
