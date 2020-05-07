use crate::{config::Config, package::Package, repo::Repo};
use anyhow::{anyhow, Context, Result};
use log::{debug, error};
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(not(unix))]
use std::os::windows::fs::symlink_read as symlink;
use std::{io::ErrorKind, process::Command};

pub fn install() -> Result<()> {
    // Get the config
    let config = Config::from_default_file_or_new()?;

    // Get the repository from the config
    let repo = Repo::new(config.clone())?;

    // Read the packages from the repository
    let packages = repo.read().context("reading packages to install")?;

    println!("Checking which packages haven't been installed yet..");
    let packages_to_install: Vec<&Package> = packages
        .iter()
        // Only keep packages where we have the package manager of
        .filter(|package| package.is_available())
        // Only keep packages that can actually be installed
        .filter(|package| !package.is_installed().expect("Could not perform command"))
        // Make it a vector again
        .collect::<_>();

    let package_names = packages_to_install
        .iter()
        // Get the names
        .map(|package| package.color_full_name())
        // Make it a vector again
        .collect::<Vec<String>>();

    let package_names = package_names
        // Convert the String into a &str
        .iter()
        .map(|name| name.as_str())
        .collect::<Vec<&str>>();

    // If there's nothing to install just return
    if package_names.is_empty() {
        println!("Nothing to install.");
        return Ok(());
    }

    // Prompt the user for which package to install
    let selections = dialoguer::MultiSelect::new()
        .with_prompt("Select the packages you want to install (space to add)")
        .items(&package_names[..])
        .interact()
        .context("failed constructing checkboxes")?;

    // Install the selected packages
    for selection in selections {
        let package = packages_to_install[selection];
        println!("Installing: {}.", package.color_full_name());

        let install_command = package.install_command();
        debug!("Installing: {}.", install_command);

        match call(install_command.split_ascii_whitespace().collect()) {
            Ok(_) => println!("{} installed successfully.", package.color_full_name()),
            Err(err) => error!("\"{}\": {:?}", package.install_command(), err),
        };
    }

    // Create the symbolic links if applicable
    config
        .symlinks
        .iter()
        // Only create symbolic references when there's no file
        .filter(|symlink| match symlink.expanded_destination().metadata() {
            // The file exists, do nothing
            Ok(_) => false,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    // The file doesn't exist, create a symlink
                    true
                } else {
                    // Something is wrong with the path
                    error!(
                        "Symlink destination path \"{}\" has error: {}",
                        symlink.destination, err
                    );

                    // Do nothing for this path
                    false
                }
            }
        })
        // Create the full path for source and return it together with the destination
        .map(|symlink| {
            (
                config.folder_path().join(&symlink.source),
                &symlink.source,
                symlink.expanded_destination(),
            )
        })
        .for_each(|(full_source, short_source, destination)| {
            // Return an error when the file doesn't exist in the repository
            if !full_source.exists() {
                error!(
                    "Could not create symbolic reference for \"{}\", file does not exist.",
                    full_source.to_str().unwrap_or("invalid_path")
                );
                return;
            }

            println!("Creating symbolic reference for \"{}\".", short_source);

            // Create the symbolic reference
            if let Err(err) = symlink(&full_source, &destination) {
                error!(
                    "Could not create symbolic reference to \"{}\": {}",
                    destination.to_str().unwrap_or("invalid_path"),
                    err
                );
                return;
            }
        });

    Ok(())
}

fn call(command: Vec<&str>) -> Result<()> {
    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);
    for arg in iter {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }

    let result = cmd.output()?;
    // Return stderr when the command failed
    if result.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8(result.stderr)?;

        Err(anyhow!("{}", stderr))
    }
}
