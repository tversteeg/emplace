use anyhow::{anyhow, Context, Result};
use log::debug;
use std::process::{Command, Stdio};

/// Perform a command.
pub fn call(command: &str, arguments: Vec<&str>) -> Result<()> {
    debug!(
        "Performing command \"{}\" with arguments {}",
        command, arguments
    );

    let mut cmd = Command::new(command);
    cmd.args(arguments);

    // Execute the command
    let result = cmd
        .output()
        .context("executing command \"{}\" {}", command, arguments)?;

    // Return stderr as an error result when the command failed
    if result.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8(result.stderr)?;

        Err(anyhow!("{}", stderr))
    }
}
