use regex::Regex;
use std::error::Error;

use crate::package::{Package, PackageSource, Packages};

pub fn catch(line: &str) -> Result<Packages, Box<dyn Error>> {
    let mut packages = vec![];

    // Parse Cargo
    packages.append(&mut match_cargo(line)?);
    // Parse Rustup components
    packages.append(&mut match_rustup_component(line)?);

    // Parse APT
    packages.append(&mut match_apt(line)?);

    // Parse Pacman
    packages.append(&mut match_pacman(line)?);

    // Parse Snap
    packages.append(&mut match_snap(line)?);

    // Parse Chocolatey
    packages.append(&mut match_choco(line)?);

    // Parse Scoop
    packages.append(&mut match_scoop(line)?);

    // Parse Pip
    packages.append(&mut match_pip(line)?);
    // Parse Pip User
    packages.append(&mut match_pip_user(line)?);

    // Parse Pip3
    packages.append(&mut match_pip3(line)?);
    // Parse Pip3 User
    packages.append(&mut match_pip3_user(line)?);

    // Parse NPM
    packages.append(&mut match_npm(line)?);

    Ok(Packages(packages))
}

fn match_rustup_component(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::RustupComponent,
        r"rustup\s+component\s+add\s+(?P<name>\S+)+",
    )
}

fn match_cargo(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    // First try to get the "--git https://" part
    let results = match_single(
        line,
        PackageSource::Cargo,
        r"cargo\s+install\s+(?P<name>--git\s+\S+)+",
    )?;
    if !results.is_empty() {
        return Ok(results);
    }

    // If no git part is found do the normal match
    match_single(
        line,
        PackageSource::Cargo,
        r"cargo\s+install\s+(-\S+\s+)*(?P<name>\S+)+",
    )
}

fn match_apt(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_multiple(
        line,
        PackageSource::Apt,
        r"apt(-get)?\s+(-\S+\s+)*install\s+(-\S+\s+)*(?P<name>[.\w\s-]+)",
        r"^([[:alpha:]]\S*)",
    )
}

fn match_pacman(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_multiple(
        line,
        PackageSource::Pacman,
        r"pacman\s+-Sy?\s+(-\S+\s+)*(?P<name>[.\w\s-]+)",
        r"([[:word:]]\S*)",
    )
}

fn match_snap(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_multiple(
        line,
        PackageSource::Snap,
        r"snap\s+install\s+(-\S+\s+)*(?P<name>[.\w\s-]+)",
        r"([[:word:]]\S*)",
    )
}

fn match_choco(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::Chocolatey,
        r"choco\S*\s+install\s+(?P<name>\S+)+",
    )
}

fn match_scoop(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::Scoop,
        r"scoop\S*\s+install\s+(-\S+\s+)*(?P<name>\S+)+",
    )
}

fn match_pip(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::Pip,
        r"pip\s+install\s+(?P<name>\w\S+)+",
    )
}

fn match_pip_user(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::PipUser,
        r"pip\s+(install\s+--user|--user\s+install)\s+(?P<name>\w\S+)+",
    )
}

fn match_pip3(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::Pip3,
        r"pip3\s+install\s+(?P<name>\w\S+)+",
    )
}

fn match_pip3_user(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::Pip3User,
        r"pip3\s+(install\s+--user|--user\s+install)\s+(?P<name>\w\S+)+",
    )
}

fn match_npm(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    match_single(
        line,
        PackageSource::Npm,
        r"npm\s+(install\s+(-g|--global)|(-g|--global)\s+install)\s+(?P<name>\S+)+",
    )
}

fn match_single(
    line: &str,
    source: PackageSource,
    command_regex: &str,
) -> Result<Vec<Package>, Box<dyn Error>> {
    Ok(Regex::new(command_regex)?
        .captures_iter(line)
        .map(|capture| Package::new(source, capture["name"].to_string()))
        .collect::<_>())
}

fn match_multiple(
    line: &str,
    source: PackageSource,
    command_regex: &str,
    arg_regex: &str,
) -> Result<Vec<Package>, Box<dyn Error>> {
    let command_re: Regex = Regex::new(command_regex)?;
    let arg_re: Regex = Regex::new(arg_regex)?;

    let mut result = vec![];
    for multiple_capture in command_re.captures_iter(line) {
        let mut multiple_vec = multiple_capture["name"]
            .split_whitespace()
            .filter_map(|capture| arg_re.captures(capture))
            .map(|package| Package::new(source, package[0].to_string()))
            .collect::<_>();
        result.append(&mut multiple_vec);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_match<F>(match_func: F, command: &str)
    where
        F: FnOnce(&str) -> Result<Vec<Package>, Box<dyn Error>>,
    {
        let command = match_func(command).unwrap();
        assert_eq!(command, vec![]);
    }

    fn single_match<F>(match_func: F, result: &str, command_str: &str)
    where
        F: FnOnce(&str) -> Result<Vec<Package>, Box<dyn Error>>,
    {
        let command = match_func(command_str).unwrap();
        assert_eq!(1, command.len(), "{}", command_str);
        assert_eq!(result, command[0].name);
        // Make sure it doesn't match anything else as well
        assert_eq!(command.len(), catch(command_str).unwrap().0.len())
    }

    fn multiple_match<F>(match_func: F, results: Vec<&str>, command_str: &str)
    where
        F: FnOnce(&str) -> Result<Vec<Package>, Box<dyn Error>>,
    {
        let command = match_func(command_str).unwrap();
        assert_eq!(results.len(), command.len(), "{}", command_str);

        for (i, result) in results.into_iter().enumerate() {
            assert_eq!(result, command[i].name);
        }
    }

    #[test]
    fn test_cargo_matches() {
        // Regular invocation
        single_match(match_cargo, "test", "cargo install test");
        single_match(match_cargo, "test", "cargo install test --force");
        single_match(match_cargo, "test", "cargo install --force test");
        single_match(
            match_cargo,
            "--git https://github.com/tversteeg/emplace",
            "cargo install --git https://github.com/tversteeg/emplace",
        );
    }

    #[test]
    fn test_rustup_component_matches() {
        // Regular invocation
        single_match(
            match_rustup_component,
            "clippy",
            "rustup component add clippy",
        );
    }

    #[test]
    fn test_apt_matches() {
        // Regular invocation
        single_match(match_apt, "test", "apt install test");
        single_match(match_apt, "test", "sudo apt install test");
        single_match(match_apt, "test", "sudo apt-get install test");

        // Multiple
        multiple_match(match_apt, vec!["test", "test2"], "apt install test test2");
        multiple_match(
            match_apt,
            vec!["test", "test2", "test3"],
            "apt install test test2 test3",
        );

        // Command names
        single_match(
            match_apt,
            "lib32gfortran5-x32-cross",
            "sudo apt install lib32gfortran5-x32-cross",
        );
        single_match(match_apt, "linux-perf-5.3", "apt install linux-perf-5.3");

        // With flags
        single_match(match_apt, "test", "sudo apt -qq install test");

        // Flags shouldn't trigger
        no_match(match_apt, "apt install -f");
        single_match(match_apt, "test", "sudo apt install test -f");
    }

    #[test]
    fn test_pacman_matches() {
        multiple_match(match_pacman, vec!["test", "test2"], "pacman -Sy test test2");

        no_match(match_pacman, "sudo snap install tor-middle-relay");
    }

    #[test]
    fn test_scoop_matches() {
        single_match(match_scoop, "test", "scoop install test");
        single_match(match_scoop, "test", "scoop install -g test");
    }

    #[test]
    fn test_snap_matches() {
        multiple_match(
            match_snap,
            vec!["test", "test2"],
            "sudo snap install test test2",
        );
    }

    #[test]
    fn test_pip_matches() {
        // Regular invocation
        single_match(match_pip, "test", "pip install test");
        single_match(match_pip_user, "test", "pip --user install test");
        single_match(match_pip_user, "test", "pip install --user test");
        single_match(match_pip3, "test", "pip3 install test");
        single_match(match_pip3_user, "test", "pip3 install --user test");

        // Shouldn't match
        no_match(match_pip, "pip install --user test");
        no_match(match_pip, "pip3 install test");
        no_match(match_pip_user, "pip install test");
        no_match(match_pip_user, "pip3 install --user test");
        no_match(match_pip3, "pip3 install --user test");
        no_match(match_pip3, "pip install test");
        no_match(match_pip3_user, "pip3 install test");
        no_match(match_pip3_user, "pip install --user test");
    }

    #[test]
    fn test_npm_matches() {
        // Regular invocation
        single_match(match_npm, "test", "npm install -g test");
        single_match(match_npm, "test", "npm -g install test");
        single_match(match_npm, "test", "npm install --global test");
        single_match(match_npm, "test", "npm --global install test");

        // Shouldn't match
        no_match(match_npm, "npm install test");
    }
}
