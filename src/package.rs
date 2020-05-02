use crate::package_manager::PackageManager;
use itertools::Itertools;
use std::{iter, ops::Deref, string::String};

#[derive(Debug, Clone)]
pub struct Package {
    /// The package manager this package belongs to.
    source: PackageManager,
    /// Name of this package.
    name: String,
    /// A list of command line flags this package should be installed with.
    flags: Vec<String>,
}

impl Package {
    /// Instantiate.
    pub fn new(source: PackageManager, name: String, flags: Vec<String>) -> Self {
        Self {
            source,
            name,
            flags,
        }
    }

    /// Only the package name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The name including the flags.
    pub fn full_command(&self) -> String {
        self.flags.iter().chain(iter::once(&self.name)).join(" ")
    }
}

#[derive(Debug, Clone)]
pub struct Packages(Vec<Package>);

impl Packages {
    /// Parse a line into a list of packages.
    pub fn from_line(line: &str) -> Self {
        // First we split the line into separating characters
        let lines = line
            .split(|c| c == ';' || c == '|' || c == '&')
            // Then try to find the proper package manager for each line, this also filters out
            // lines that are not related to the package manager
            .filter_map(|line| {
                // Filter out matches with less than 4 characters, it's impossible to install a
                // package that we can catch like that
                if line.len() < 4 {
                    return None;
                }

                // Attempt to find a matching package manager with the line
                match PackageManager::from_line(line) {
                    // Pass the line along
                    Some(manager) => Some((line, manager)),
                    None => None,
                }
            })
            // Parse the packages in the line with the package manager supplied
            .map(|(line, package_manager)| package_manager.catch(line))
            // Create a long list of the list of lists
            .flatten()
            .collect();

        Self(lines)
    }
}

impl Deref for Packages {
    type Target = Vec<Package>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Package>> for Packages {
    fn from(x: Vec<Package>) -> Self {
        Packages(x)
    }
}
