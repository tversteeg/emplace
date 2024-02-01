use crate::{
    package::Package,
    package_manager::{CaptureFlag, PackageInstalledMethod, PackageManager, PackageManagerTrait},
};
use anyhow::{Context, Result};
use itertools::iproduct;
use run_script::ScriptOptions;
use std::{env::split_paths, iter::Peekable, path::PathBuf};
use strum::IntoEnumIterator;

impl PackageManager {
    /// Whether the line contains a package manager.
    pub fn detects_line(line: &str) -> bool {
        Self::single_from_line(line).is_some()
    }

    /// Try to find the best matching package manager corresponding to a line.
    pub fn single_from_line(line: &str) -> Option<Self> {
        // Iterate over all enum variations
        Self::iter().find(|manager| {
            // Iterate over all commands
            manager
                .os_commands()
                .into_iter()
                // Find the command that's in the file, use an extra space to only match full
                // package names
                .any(|command| Self::line_contains_command(line, &command))
        })
    }

    /// Get all possible package manager corresponding to a line.
    pub fn from_line_iter(line: &str) -> impl Iterator<Item = Self> + '_ {
        // Iterate over all enum variations
        Self::iter().filter(move |manager| {
            // Iterate over all commands
            manager
                .os_commands()
                .into_iter()
                // Find the command that's in the file, use an extra space to only match full
                // package names
                .any(|command| Self::line_contains_command(line, &command))
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
        // Taking all paths, merging them with executable name and checking, that executable exists.
        let paths: Vec<PathBuf> = split_paths(&path).collect();

        // Create a cartesian product of (Path, executable_name) and checking if any of pairs exists
        iproduct!(paths, self.os_commands())
            .map(|(path, exec)| path.join(PathBuf::from(exec)).exists())
            .any(|x| x)
    }

    /// Extract the packages from the line.
    pub fn catch(self, line: &str) -> Vec<Package> {
        // Try all different commands
        self.os_commands()
            .iter()
            .flat_map(|command| {
                // If the command can't be found in this line just continue
                if !Self::line_contains_command(line, command) {
                    return vec![];
                }

                // Get the part right of the package manager invocation
                // The command has another space so lengthened versions of itself don't collide,
                // for example 'apt' & 'apt-get'
                match line.split(&format!("{} ", command)).nth(1) {
                    Some(rest_of_line) => {
                        // The resulting packages strings
                        let mut package_strings = vec![];

                        // A list of flags that we caught that we should keep track of
                        let mut catched_flags = vec![];

                        // Get the line with the subcommand removed
                        let line_without_subcommand = self
                            .sub_commands()
                            .clone()
                            .into_iter()
                            .find_map(|sub_command| {
                                // Use a space terminated command to ensure that only exactly the
                                // subcommand is removed
                                let sub_command_with_space_postfix = format!("{} ", sub_command);
                                if rest_of_line.starts_with(&sub_command_with_space_postfix) {
                                    Some(
                                        rest_of_line[sub_command_with_space_postfix.len()..]
                                            .to_string(),
                                    )
                                } else {
                                    let sub_command_with_spaces =
                                        format!(" {}", sub_command_with_space_postfix);
                                    // Remove the full subcommand from the list
                                    let split_vec: Vec<_> =
                                        rest_of_line.split(&sub_command_with_spaces).collect();
                                    if split_vec.len() == 1 {
                                        // If nothing is split it means that the sub command is not
                                        // found in the string
                                        None
                                    } else {
                                        // Rejoin the string with a space where the full subcommand
                                        // with spaces was
                                        Some(split_vec.join(" "))
                                    }
                                }
                            });

                        // Return an empty array when no subcommand found
                        let line_without_subcommand = match line_without_subcommand {
                            Some(line_without_subcommand) => line_without_subcommand,
                            None => return vec![],
                        };

                        // Convert the line into an iterator over all arguments delimited by
                        // whitespace
                        let mut args_iter =
                            line_without_subcommand.split_ascii_whitespace().peekable();

                        // Loop over the arguments handling flags in a special way
                        while let Some(arg) = args_iter.next() {
                            // Stop when a flag is found that invalidate the command
                            if self.has_invalidating_flag(arg) {
                                return vec![];
                            }

                            let first_char = arg
                                .chars()
                                .next()
                                .expect("Arg string is suddenly zero bytes");

                            if first_char == '-' || first_char == '+' {
                                self.handle_capture_flags(arg, &mut args_iter, &mut catched_flags);

                                // If it's a flag containing an extra arguments besides it skip one
                                if self.known_flags_with_values().contains(&arg) {
                                    // Skip the next item
                                    args_iter.next();
                                    continue;
                                }
                            // Only match packages starting with no special characters
                            } else if first_char.is_alphanumeric() {
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
            .collect()
    }

    /// Get OS specific commands, add .exe & .cmd on Windows.
    #[cfg(target_os = "windows")]
    fn os_commands(&self) -> Vec<String> {
        self.commands()
            .into_iter()
            .map(|command| {
                vec![
                    command.to_string(),
                    format!("{}.exe", command),
                    format!("{}.cmd", command),
                ]
            })
            .flatten()
            .collect()
    }

    /// Get OS specific commands.
    #[cfg(not(target_os = "windows"))]
    fn os_commands(&self) -> Vec<String> {
        self.commands().into_iter().map(|s| s.to_string()).collect()
    }

    /// Whether the command has an invalidating flag.
    fn has_invalidating_flag(self, arg: &str) -> bool {
        self.invalidating_flags()
            .iter()
            .any(|capture| capture == &arg)
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

    /// Whether a line can be parsed with a command.
    fn line_contains_command(line: &str, command: &str) -> bool {
        line.lines().any(|line| {
            let command_with_space = format!("{} ", command);

            // Only match `pkg` for example, but not `bpkg`
            line.starts_with(&command_with_space)
                || line.contains(&format!(" {}", command_with_space))
                || line.contains(&format!("\t{}", command_with_space))
        })
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

        assert!(PackageManager::detects_line("echo test\napt install test"));
        assert!(PackageManager::detects_line(
            "apt install test\n\tapt install test"
        ));
        assert!(PackageManager::detects_line("\n\tapt install test"));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_detect_windows() {
        assert!(PackageManager::detects_line("scoop.exe -h"));
        assert!(PackageManager::detects_line("scoop.cmd -h"));
    }
}
