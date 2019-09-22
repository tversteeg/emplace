use regex::Regex;
use std::error::Error;

use crate::package::{Package, Packages, PackageSource};

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
        static ref APT_RE: Regex = Regex::new(r"apt\S*\s+install\s+(?P<name>\S+)+").unwrap();
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
