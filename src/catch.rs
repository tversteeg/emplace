use crate::{config::Config, package::Packages, package_manager::PackageManager, repo::Repo};
use anyhow::Result;
use colored::Colorize;

pub fn catch(line: &str) -> Result<()> {
    // Do a quick check so it won't stall the terminal
    if !PackageManager::detects_line(line) {
        return Ok(());
    }

    // Get the packages from this line
    let mut catches = Packages::from_line(line);

    // Nothing found, just return
    if catches.is_empty() {
        return Ok(());
    }

    // Get the config
    let config = Config::from_default_file_or_new()?;

    // Get the repository from the config
    let repo = Repo::new(config)?;

    // Only keep the packages that haven't been saved already
    catches.filter_saved_packages(&repo.read()?);

    let len = catches.len();
    if len == 0 {
        // Nothing found after filtering
        return Ok(());
    }

    // Print the list of things that need to be catched
    match len {
        1 => println!("{}", "Mirror this command?".green().bold()),
        n => println!("{}", format!("Mirror these {} commands?", n).green().bold()),
    }
    for catch in catches.iter() {
        println!("- {}", catch.color_full_name());
    }

    // Ask if it needs to be mirrored
    if !dialoguer::Confirm::new().interact()? {
        // Exit, we don't need to do anything
        return Ok(());
    }

    repo.mirror(catches)?;

    Ok(())
}
