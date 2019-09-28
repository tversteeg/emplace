use regex::Regex;
use std::error::Error;

use crate::package::{Package, Packages, PackageSource};

pub fn catch(line: &str) -> Result<Packages, Box<dyn Error>> {
    let mut packages = vec![];

    // Parse Cargo
    packages.append(&mut match_cargo(line)?);

    // Parse APT
    packages.append(&mut match_apt(line)?);

    // Parse Chocolatey
    packages.append(&mut match_choco(line)?);

    Ok(Packages(packages))
}

fn match_cargo(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref CARGO_RE: Regex = Regex::new(r"cargo\s+install\s+(?P<name>\S+)+").unwrap();
    }

    let mut packages = vec![];

    for capture in CARGO_RE.captures_iter(line) {
        packages.push(Package::new(PackageSource::Cargo, capture["name"].to_string()));
    }

    Ok(packages)
}

fn match_apt(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref APT_RE: Regex = Regex::new(r"apt(-get)?\s+(-\S+\s+)*install\s+(-\S+\s+)*(?P<name>\S+)").unwrap();
    }

    let mut packages = vec![];

    for capture in APT_RE.captures_iter(line) {
        packages.push(Package::new(PackageSource::Apt, capture["name"].to_string()));
    }

    Ok(packages)
}

fn match_choco(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref CHOCO_RE: Regex = Regex::new(r"choco\S*\s+install\s+(?P<name>\S+)+").unwrap();
    }

    let mut packages = vec![];

    for capture in CHOCO_RE.captures_iter(line) {
        packages.push(Package::new(PackageSource::Chocolatey, capture["name"].to_string()));
    }

    Ok(packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_match<F>(match_func: F, result: &str, command: &str) where
        F: FnOnce(&str) -> Result<Vec<Package>, Box<dyn Error>>
    {
        let command = match_func(command).unwrap();
        assert_eq!(1, command.len());
        assert_eq!(result, command[0].name)
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

        // Command names
        single_match(match_apt, "lib32gfortran5-x32-cross", "sudo apt install lib32gfortran5-x32-cross");

        // With flags
        single_match(match_apt, "test", "sudo apt -qq install test");
        //single_apt_match("test", "sudo apt install -t experimental test");
    }
}
