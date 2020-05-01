mod apt;
#[macro_use]
pub mod test_macro;

use crate::package::Package;
use anyhow::{Context, Result};
use run_script::ScriptOptions;
use std::{fmt, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Enum containing all package managers.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum PackageManager {
    Apt,
}

impl PackageManager {
    /// Whether the line contains a package manager.
    pub fn detects_line(line: &str) -> bool {
        Self::from_line(line).is_some()
    }

    /// Try to find the proper package manager corresponding to a line.
    pub fn from_line(line: &str) -> Option<Self> {
        // Iterate over all enum variations
        Self::iter().find(|manager| {
            // Iterate over all commands
            manager
                .get()
                .commands()
                .into_iter()
                // Find the command that's in the file
                .find(|command| line.contains(command))
                .is_some()
        })
    }

    /// Check whether a package is already installed.
    pub fn is_installed(self, package: &str) -> Result<bool> {
        match self.get().is_installed(package) {
            PackageInstalledMethod::Script(script) => {
                // Run the installation script
                let mut options = ScriptOptions::new();
                options.exit_on_error = true;
                options.print_commands = false;

                // Only catch the exit status of the script
                let (code, _, _) = run_script::run(&script, &vec![], &options)
                    .context("could not check whether package is installed")?;

                Ok(code == 0)
            }
            PackageInstalledMethod::Path(path) => {
                // Check whether the file or directory exists
                Ok(path.exists())
            }
        }
    }

    /// Extract the packages from the line.
    pub fn catch(self, line: &str) -> Vec<Package> {
        let manager = self.get();

        // Try all different commands
        manager
            .commands()
            .iter()
            .map(|command| {
                // Get the part right of the package manager invocation
                // The command has another space so lengthened versions of itself don't collide,
                // for example 'apt' & 'apt-get'
                match line.split(&format!("{} ", command)).skip(1).next() {
                    Some(rest_of_line) => {
                        // Split all arguments
                        let mut args_iter = rest_of_line.split_ascii_whitespace();

                        // The resulting packages strings
                        let mut package_strings = vec![];

                        // A list of flags that we caught that we should keep track of
                        let mut catched_flags = vec![];

                        // Keep track of whether the installation subcommand is present
                        let mut has_sub_command = false;

                        // Loop over the arguments handling flags in a special way
                        while let Some(arg) = args_iter.next() {
                            // Ignore the sub command
                            if !has_sub_command && manager.sub_commands().contains(&arg) {
                                has_sub_command = true;
                                continue;
                            }

                            // Check if the argument is a flag
                            if arg.chars().nth(0) == Some('-') {
                                // If it's a flag we need to capture keep track of it
                                if manager.capture_flags().contains(&arg) {
                                    catched_flags.push(arg.to_string());
                                }

                                // If it's a flag containing an extra arguments besides it skip one
                                if manager.known_flags_with_values().contains(&arg) {
                                    // Skip the next item
                                    args_iter.next();
                                    continue;
                                }
                            } else {
                                // We've found a package
                                package_strings.push(arg.to_string());
                            }
                        }

                        // No installation subcommand means no packages
                        if !has_sub_command {
                            return vec![];
                        }

                        // Now convert it into actual packages
                        package_strings
                            .into_iter()
                            .map(|name| Package::new(self, name, catched_flags.clone()))
                            .collect()
                    }
                    // Package manager command was the last word of the line
                    None => vec![],
                }
            })
            .flatten()
            .collect()
    }

    /// Return the package manager trait object depending on the enum variant.
    fn get(self) -> impl PackageManagerTrait {
        match self {
            PackageManager::Apt => apt::Apt,
        }
    }
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.get().full_name())
    }
}

/// Trait that needs to be implemented for a new package manager.
pub trait PackageManagerTrait:
    fmt::Debug + Default + Copy + Clone + Eq + PartialEq + Sized
{
    /// A descriptive name.
    fn full_name(self) -> &'static str;

    /// The command-line word(s) to invoke the package manager.
    fn commands(self) -> Vec<&'static str>;

    /// The command-line subcommand(s) that's used to catch installing new packages.
    fn sub_commands(self) -> Vec<&'static str>;

    /// Command that's used to install new packages.
    fn install_command(self) -> &'static str;

    /// Does installing a new command need administrative permissions?
    fn needs_root(self) -> bool;

    /// Check whether a package is already installed.
    fn is_installed(self, package: &str) -> PackageInstalledMethod;

    /// A list of known command line flags that accept an extra argument which could be the name of
    /// the package.
    fn known_flags_with_values(self) -> Vec<&'static str>;

    /// A list of command line flags that should be caught as well.
    ///
    /// This should usually stay empty.
    fn capture_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

/// The way a package is checked if it's installed.
pub enum PackageInstalledMethod {
    /// A simple command line script will check if the package is installed.
    Script(String),
    /// A file or directory on the filesystem is checked for existence.
    Path(PathBuf),
}
