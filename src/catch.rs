use crate::{package::Packages, package_manager::PackageManager};
use anyhow::Result;

pub fn catch(line: &str) -> Result<()> {
    // Do a quick check so it won't stall the terminal
    if !PackageManager::detects_line(line) {
        return Ok(());
    }

    // Get the packages from this line
    let packages = Packages::from_line(line);

    Ok(())
}
