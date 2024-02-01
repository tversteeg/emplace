use crate::package_manager::{PackageManager, PackageManagerTrait};
use anyhow::Result;
use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    iter::{self, IntoIterator},
    ops::Deref,
    string::String,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
    /// The package manager this package belongs to.
    source: PackageManager,
    /// Name of this package.
    name: String,
    /// A list of command line flags this package should be installed with.
    #[serde(default)]
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

    /// The full command needed to install this package.
    #[cfg(not(target_os = "windows"))]
    pub fn install_command(&self) -> String {
        if self.source.needs_root() {
            format!(
                "sudo {} {}",
                self.source.install_command(),
                self.full_command()
            )
        } else {
            format!("{} {}", self.source.install_command(), self.full_command())
        }
    }

    /// The full command needed to install this package.
    #[cfg(target_os = "windows")]
    pub fn install_command(&self) -> String {
        format!("{} {}", self.source.install_command(), self.full_command())
    }

    /// The full name in fancy colors.
    pub fn color_full_name(&self) -> String {
        if self.flags.is_empty() {
            format!(
                "{} ({})",
                self.name.yellow(),
                self.source.full_name().green()
            )
        } else {
            format!(
                "{} {} ({})",
                self.flags.iter().join(" ").dimmed(),
                self.name.yellow(),
                self.source.full_name().green(),
            )
        }
    }

    /// The command line flags.
    ///
    /// Used by the test_macro.
    #[allow(unused)]
    pub fn flags(&self) -> &Vec<String> {
        &self.flags
    }

    /// Check if this package is already installed.
    pub fn is_installed(&self) -> Result<bool> {
        self.source.package_is_installed(self)
    }

    /// Check if the package manager can be found.
    pub fn is_available(&self) -> bool {
        self.source.is_available()
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_command().cmp(&other.full_command())
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packages(Vec<Package>);

impl Packages {
    /// An empty list.
    pub fn empty() -> Self {
        Self(vec![])
    }

    /// Parse a line into a list of packages.
    pub fn from_line(line: &str) -> Self {
        // First we split the line into separating characters
        let lines = line
            .split(|c| c == ';' || c == '|' || c == '&' || c == '\r' || c == '\n')
            // Then try to find the proper package manager for each line, this also filters out
            // lines that are not related to the package manager
            .flat_map(|line| {
                // Attempt to find a matching package managers with the line
                PackageManager::from_line_iter(line).map(move |manager| (line, manager))
            })
            // Parse the packages in the line with the package manager supplied
            .flat_map(|(line, package_manager)| package_manager.catch(line))
            .collect();

        Self(lines)
    }

    /// Get the union of this and another list of packages.
    pub fn merge(&mut self, other: &mut Packages) {
        // Add the other packages
        self.0.append(&mut other.0);

        // Sort them so we can remove deduplicates
        self.0.sort();
        // Remove the duplicates
        self.0.dedup();
    }

    /// Remove all packages that have been saved already.
    pub fn filter_saved_packages(&mut self, old: &Packages) {
        self.0
            .retain(|package| !old.iter().any(|old_package| package == old_package));
    }

    /// Remove all duplicate packages.
    pub fn dedup(&mut self) {
        self.0.dedup();
    }

    /// Construct a commit message depending on the amount of packages that need to be committed.
    pub fn commit_message(&self) -> String {
        match self.0.len() {
            0 => panic!("Can't create a commit message for empty changes"),
            1 => format!("Emplace - mirror package \"{}\"", self.0[0].full_command()),
            n => format!("Emplace - mirror {} packages", n),
        }
    }
}

impl IntoIterator for Packages {
    type Item = Package;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
