use std::{
    error::Error,
    process::Command
};

use crate::package::{Package, Packages};

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
    println!("Checking which packages haven't been installed yet..");
    let packages_to_install: Vec<&Package> = packages.iter()
        // Only keep packages that can actually be installed
        .filter(|package| call(package.is_installed_command(), false).expect("Could not perform command"))
        // Make it a vector again
        .collect::<_>();

    let package_names: Vec<String> = packages_to_install.iter()
        // Get the names
        .map(|package| format!("{}", package))
        // Make it a vector again
        .collect::<_>();
    let package_names: Vec<&str> = package_names.iter().map(|name| name.as_str()).collect::<_>();

    let selections = dialoguer::Checkboxes::new()
        .with_prompt("Select the packages you want to install")
        .items(&package_names[..])
        .interact()?;

    for selection in selections {
        let package = packages_to_install[selection];
        println!("\nInstalling \"{}\"", package);

        call(package.install_command(), false)?;

        println!("Installed.");
    }

    Ok(())
}
