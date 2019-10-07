use regex::Regex;
use std::error::Error;

use crate::package::{Package, PackageSource, Packages};

pub fn catch(line: &str) -> Result<Packages, Box<dyn Error>> {
    let mut packages = vec![];

    // Parse Cargo
    packages.append(&mut match_cargo(line)?);

    // Parse APT
    packages.append(&mut match_apt(line)?);

    // Parse Chocolatey
    packages.append(&mut match_choco(line)?);

    // Parse Pip
    packages.append(&mut match_pip(line)?);
    // Parse Pip User
    packages.append(&mut match_pip_user(line)?);

    // Parse Pip3
    packages.append(&mut match_pip3(line)?);
    // Parse Pip3 User
    packages.append(&mut match_pip3_user(line)?);

    Ok(Packages(packages))
}

fn match_cargo(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref CARGO_RE: Regex = Regex::new(r"cargo\s+install\s+(?P<name>\S+)+").unwrap();
    }
    Ok(CARGO_RE
        .captures_iter(line)
        .map(|capture| Package::new(PackageSource::Cargo, capture["name"].to_string()))
        .collect::<_>())
}

fn match_apt(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref APT_RE: Regex =
            Regex::new(r"apt(-get)?\s+(-\S+\s+)*install\s+(-\S+\s+)*(?P<name>[\w\s-]+)").unwrap();
        static ref APT_MULTIPLE_RE: Regex = Regex::new(r"([[:word:]]\S*)").unwrap();
    }
    let mut result = vec![];
    for multiple_capture in APT_RE.captures_iter(line) {
        let multiple_iter = APT_MULTIPLE_RE.captures_iter(&multiple_capture["name"]);
        let mut multiple_vec: Vec<Package> = multiple_iter
            .map(|capture| Package::new(PackageSource::Apt, capture[0].to_string()))
            .collect::<_>();
        result.append(&mut multiple_vec);
    }
    Ok(result)
}

fn match_choco(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref CHOCO_RE: Regex = Regex::new(r"choco\S*\s+install\s+(?P<name>\S+)+").unwrap();
    }
    Ok(CHOCO_RE
        .captures_iter(line)
        .map(|capture| Package::new(PackageSource::Chocolatey, capture["name"].to_string()))
        .collect::<_>())
}

fn match_pip(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref PIP_RE: Regex = Regex::new(r"pip\s+install\s+(?P<name>\S+)+").unwrap();
    }
    Ok(PIP_RE
        .captures_iter(line)
        .map(|capture| Package::new(PackageSource::Pip, capture["name"].to_string()))
        .collect::<_>())
}

fn match_pip_user(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref PIP_USER_RE: Regex =
            Regex::new(r"pip\s+--user\s+install\s+(?P<name>\S+)+").unwrap();
    }
    Ok(PIP_USER_RE
        .captures_iter(line)
        .map(|capture| Package::new(PackageSource::PipUser, capture["name"].to_string()))
        .collect::<_>())
}

fn match_pip3(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref PIP3_RE: Regex = Regex::new(r"pip3\s+install\s+(?P<name>\S+)+").unwrap();
    }
    Ok(PIP3_RE
        .captures_iter(line)
        .map(|capture| Package::new(PackageSource::Pip3, capture["name"].to_string()))
        .collect::<_>())
}

fn match_pip3_user(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref PIP3_USER_RE: Regex =
            Regex::new(r"pip3\s+--user\s+install\s+(?P<name>\S+)+").unwrap();
    }
    Ok(PIP3_USER_RE
        .captures_iter(line)
        .map(|capture| Package::new(PackageSource::Pip3User, capture["name"].to_string()))
        .collect::<_>())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_match<F>(match_func: F, command: &str)
    where
        F: FnOnce(&str) -> Result<Vec<Package>, Box<dyn Error>>,
    {
        let command = match_func(command).unwrap();
        assert_eq!(0, command.len());
    }

    fn single_match<F>(match_func: F, result: &str, command: &str)
    where
        F: FnOnce(&str) -> Result<Vec<Package>, Box<dyn Error>>,
    {
        let command = match_func(command).unwrap();
        assert_eq!(1, command.len());
        assert_eq!(result, command[0].name)
    }

    fn multiple_match<F>(match_func: F, results: Vec<&str>, command: &str)
    where
        F: FnOnce(&str) -> Result<Vec<Package>, Box<dyn Error>>,
    {
        let command = match_func(command).unwrap();
        assert_eq!(results.len(), command.len());

        let mut i: usize = 0;
        for result in results.into_iter() {
            assert_eq!(result, command[i].name);
            i = i + 1;
        }
    }

    #[test]
    fn test_cargo_matches() {
        // Regular invocation
        single_match(match_cargo, "test", "cargo install test");
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

        // With flags
        single_match(match_apt, "test", "sudo apt -qq install test");
    }

    #[test]
    fn test_pip_matches() {
        // Regular invocation
        single_match(match_pip, "test", "pip install test");
        single_match(match_pip_user, "test", "pip --user install test");
        single_match(match_pip3, "test", "pip3 install test");
        single_match(match_pip3_user, "test", "pip3 --user install test");

        // Shouldn't match
        no_match(match_pip, "pip --user test");
        no_match(match_pip, "pip3 test");
        no_match(match_pip_user, "pip test");
        no_match(match_pip_user, "pip3 --user test");
        no_match(match_pip3, "pip3 --user test");
        no_match(match_pip3, "pip test");
        no_match(match_pip3_user, "pip3 test");
        no_match(match_pip3_user, "pip --user test");
    }
}
