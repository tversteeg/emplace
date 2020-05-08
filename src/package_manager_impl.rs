use crate::{
    package::Package,
    package_manager::{CaptureFlag, PackageInstalledMethod, PackageManager, PackageManagerTrait},
};
use anyhow::{Context, Result};
#[macro_use]
use itertools::{Itertools, iproduct};
use is_executable::is_executable;
use run_script::ScriptOptions;
use std::env::split_paths;
use std::iter::Peekable;
use std::path::PathBuf;
use strum::IntoEnumIterator;

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
                .commands()
                .into_iter()
                // Find the command that's in the file, use an extra space to only match full
                // package names
                .any(|command| line.contains(&format!("{} ", command)))
        })
    }

    /// Check whether a package is already installed.
    pub fn package_is_installed(self, package: &Package) -> Result<bool> {
        match self.is_installed(package.name()) {
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

    /// Check if this package manager is available.
    pub fn is_available(self) -> bool {
        let path = std::env::var_os("PATH").expect("PATH env is not set");
        //taking all paths, merging them with executable name and checking, that executable exists.

        let paths: Vec<PathBuf> = split_paths(&path).collect();
        //creating cartesian product  of (Path, executable_name) and checking
        // if any of pairs exists
        iproduct!(paths, self.commands())
            .map(|(path, exec): (PathBuf, &str)| path.join(PathBuf::from(exec)))
            .map(is_executable)
            .any(|x| x)
    }

    /// Extract the packages from the line.
    pub fn catch(self, line: &str) -> Vec<Package> {
        // Try all different commands
        self.commands()
            .iter()
            .map(|command| {
                // Get the part right of the package manager invocation
                // The command has another space so lengthened versions of itself don't collide,
                // for example 'apt' & 'apt-get'
                match line.split(&format!("{} ", command)).nth(1) {
                    Some(rest_of_line) => {
                        // The resulting packages strings
                        let mut package_strings = vec![];

                        // A list of flags that we caught that we should keep track of
                        let mut catched_flags = vec![];

                        // Get which sub command is used
                        let sub_command = self
                            .sub_commands()
                            .clone()
                            .into_iter()
                            .find(|command| rest_of_line.contains(&format!("{} ", command)));

                        // Remove the sub command from the line
                        let line_without_subcommand = match sub_command {
                            Some(command) => rest_of_line.split(command).join(""),
                            // No installation subcommand means no packages
                            None => return vec![],
                        };

                        // Convert the line into an iterator over all arguments delimited by
                        // whitespace
                        let mut args_iter =
                            line_without_subcommand.split_ascii_whitespace().peekable();

                        // Loop over the arguments handling flags in a special way
                        while let Some(arg) = args_iter.next() {
                            if arg.starts_with('-') {
                                self.handle_capture_flags(&arg, &mut args_iter, &mut catched_flags);

                                // If it's a flag containing an extra arguments besides it skip one
                                if self.known_flags_with_values().contains(&arg) {
                                    // Skip the next item
                                    args_iter.next();
                                    continue;
                                }
                            } else {
                                // We've found a package
                                package_strings.push(arg.to_string());
                            }
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

    /// Handle the iterator's flags using the different options as defined in the package managers.
    fn handle_capture_flags<'a, I>(
        self,
        arg: &str,
        args_iter: &mut Peekable<I>,
        catched_flags: &mut Vec<String>,
    ) where
        I: Iterator<Item = &'a str>,
    {
        // Find the matching flags from the capture_flags function
        let capture = match self
            .capture_flags()
            .into_iter()
            .find(|capture| capture.flag() == arg)
        {
            Some(capture) => capture,
            None => return,
        };

        match capture {
            CaptureFlag::Single(flag) => {
                // Just a single flag, add it to the list
                catched_flags.push(flag.to_string());
            }
            CaptureFlag::SetValue(flag, value) => {
                // The value is set and must match
                if let Some(next_arg) = args_iter.peek() {
                    if &value == next_arg {
                        catched_flags.push(format!("{} {}", flag, value));
                        // We've looked at the next item so we should also skip it
                        args_iter.next();
                    }
                }
            }
            CaptureFlag::DynamicValue(flag) => {
                // The flag matches and the next value is dynamic so just take that
                if let Some(next_arg) = args_iter.next() {
                    catched_flags.push(format!("{} {}", flag, next_arg))
                }
            }
        }
    }
}

impl CaptureFlag {
    /// Extract the flag which is always there.
    pub fn flag(self) -> &'static str {
        match self {
            CaptureFlag::Single(flag) => flag,
            CaptureFlag::SetValue(flag, _) => flag,
            CaptureFlag::DynamicValue(flag) => flag,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::package_manager::PackageManager;

    #[test]
    fn test_detect() {
        assert!(PackageManager::detects_line("apt install test"));
        assert!(!PackageManager::detects_line("something"));
    }
}
