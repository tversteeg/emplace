use crate::{
    config::Config,
    package::{Package, Packages},
    repo::Repo,
};
use anyhow::{Context, Result};

/// Remove listed packages.
pub fn clean() -> Result<()> {
    // Get the config
    let config = Config::from_default_file_or_new()?;

    // Get the repository from the config
    let repo = Repo::new(config, true)?;

    // Read the packages from the repository
    let packages = repo.read().context("reading packages to clean")?;

    // Get the packages that need to be cleaned
    let leftover_packages = packages_to_clean(packages).context("selecting packages to clean")?;

    // Set the new list
    repo.clean(leftover_packages)
}

/// List all packages minus the ones the user selected to clean.
fn packages_to_clean(packages: Packages) -> Result<Packages> {
    let package_names: Vec<String> = packages
        .iter()
        // Get the names
        .map(|package| package.color_full_name())
        // Make it a vector again
        .collect::<_>();
    let package_names: Vec<&str> = package_names
        .iter()
        .map(|name| name.as_str())
        .collect::<_>();

    // If there's nothing to remove just return
    if package_names.is_empty() {
        println!("No packages have been added yet.");
        return Ok(packages);
    }

    let selections = dialoguer::MultiSelect::new()
        .with_prompt("Select the packages you want to stop syncing (space to select)")
        .items(&package_names[..])
        .interact()
        .context("failed constructing checkboxes")?;

    // Get the all the packages without the filtered ones
    Ok(packages
        .iter()
        .enumerate()
        .filter_map(|(index, package)| {
            for selection in &selections {
                if index == *selection {
                    // Remove the filtered ones
                    return None;
                }
            }
            Some(package.clone())
        })
        .collect::<Vec<Package>>()
        .into())
}
