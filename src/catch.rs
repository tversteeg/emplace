use crate::{config::Config, package::Packages, package_manager::PackageManager};
use anyhow::Result;

pub fn catch(line: &str) -> Result<()> {
    // Do a quick check so it won't stall the terminal
    if !PackageManager::detects_line(line) {
        return Ok(());
    }

    // Get the packages from this line
    let packages = Packages::from_line(line);

    // Nothing found, just return
    if packages.is_empty() {
        return Ok(());
    }

    // Get the config
    let config = Config::from_default_file_or_new()?;

    dbg!(config);

    Ok(())
}
