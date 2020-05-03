use crate::{
    package::Package,
    package_manager::{PackageInstalledMethod, PackageManager, PackageManagerTrait},
};
use anyhow::{Context, Result};
use run_script::ScriptOptions;
use std::process::{Command, Stdio};
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
                .find(|command| line.contains(&format!("{} ", command)))
                .is_some()
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
        // TODO check for the command in the path instead of executing it
        self.commands()
            .into_iter()
            .find(|command| {
                Command::new(command)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .is_ok()
            })
            .is_some()
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
                match line.split(&format!("{} ", command)).skip(1).next() {
                    Some(rest_of_line) => {
                        // Split all arguments
                        let mut args_iter = rest_of_line.split_ascii_whitespace().peekable();

                        // The resulting packages strings
                        let mut package_strings = vec![];

                        // A list of flags that we caught that we should keep track of
                        let mut catched_flags = vec![];

                        // Keep track of whether the installation subcommand is present
                        let mut has_sub_command = false;

                        // Loop over the arguments handling flags in a special way
                        while let Some(arg) = args_iter.next() {
                            // Ignore the sub command
                            if !has_sub_command && self.sub_commands().contains(&arg) {
                                has_sub_command = true;
                                continue;
                            }

                            // Check if the argument is a flag
                            if arg.chars().nth(0) == Some('-') {
                                // If it's a flag we need to capture keep track of it
                                let next_arg = args_iter.peek();
                                if let Some((flag_first, flag_second)) = self
                                    .capture_flags()
                                    .iter()
                                    // Compare the first and optionally the second flags to the
                                    // current and the next arguments
                                    .find(|(flag_first, flag_second)| {
                                        &arg == flag_first
                                            && (flag_second.is_none()
                                                || *next_arg.unwrap_or(&"")
                                                    == flag_second.unwrap_or(""))
                                    })
                                {
                                    catched_flags.push(match flag_second {
                                        Some(flag_second) => {
                                            // Skip the next item since it's the second flag
                                            args_iter.next();
                                            format!("{} {}", flag_first, flag_second)
                                        }
                                        None => format!("{}", flag_first),
                                    });

                                    // Continue since we captured the flag, don't need to do
                                    // anything with it after that
                                    continue;
                                }

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
