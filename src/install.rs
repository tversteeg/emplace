use anyhow::Context;
use log::{error, info};
use std::{
    error::Error,
    process::{Command, Stdio},
};

use crate::package::{Package, Packages};

fn can_call(command: &str) -> bool {
    Command::new(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

fn call(command: Vec<&str>, dry_run: bool) -> Result<bool, Box<dyn Error>> {
    if dry_run {
        println!("{}", command.join(" "));
        return Ok(true);
    }

    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);
    for arg in iter {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }

    let result = cmd.output()?;

    Ok(result.status.success())
}

pub fn install(packages: Packages) -> Result<(), Box<dyn Error>> {
    info!("Checking which packages haven't been installed yet..");
    let packages_to_install: Vec<&Package> = packages
        .iter()
        // Only keep packages where we have the package manager of
        .filter(|package| can_call(package.command()))
        // Only keep packages that can actually be installed
        .filter(|package| !package.is_installed().expect("Could not perform command"))
        // Make it a vector again
        .collect::<_>();

    let package_names: Vec<String> = packages_to_install
        .iter()
        // Get the names
        .map(|package| package.colour_full_name())
        // Make it a vector again
        .collect::<_>();
    let package_names: Vec<&str> = package_names
        .iter()
        .map(|name| name.as_str())
        .collect::<_>();

    // If there's nothing to install just return
    if package_names.is_empty() {
        info!("Nothing to install.");
        return Ok(());
    }

    let selections = dialoguer::Checkboxes::new()
        .with_prompt("Select the packages you want to install (space to add)")
        .items(&package_names[..])
        .interact()
        .context("failed constructing checkboxes")?;

    for selection in selections {
        let package = packages_to_install[selection];
        info!("Installing: {}.", package.colour_full_name());

        if call(package.install_command(), false)? {
            info!("{} installed successfully.", package.colour_full_name());
        } else {
            error!("{} installed successfully.", package.colour_full_name());
        }
    }

    Ok(())
}

pub fn clean(packages: Packages) -> Result<Packages, Box<dyn Error>> {
    let package_names: Vec<String> = packages
        .iter()
        // Get the names
        .map(|package| package.colour_full_name())
        // Make it a vector again
        .collect::<_>();
    let package_names: Vec<&str> = package_names
        .iter()
        .map(|name| name.as_str())
        .collect::<_>();

    // If there's nothing to remove just return
    if package_names.is_empty() {
        info!("No packages have been added yet.");
        return Ok(packages);
    }

    let selections = dialoguer::Checkboxes::new()
        .with_prompt("Select the packages you want to stop syncing (space to select)")
        .items(&package_names[..])
        .interact()
        .context("failed constructing checkboxes")?;

    // Get the all the packages without the filtered ones
    Ok(Packages(
        packages
            .iter()
            .enumerate()
            .filter_map(|(index, package)| {
                for selection in &selections {
                    if index == *selection {
                        return None;
                    }
                }
                Some(package.clone())
            })
            .collect::<_>(),
    ))
}
