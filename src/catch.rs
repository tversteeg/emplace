use regex::Regex;
use std::{
    error::Error,
    string::String,
};

pub enum PackageType {
    /// Debian apt-get.
    Apt,
    /// Windows chocolatey.
    Chocolatey,
}

impl PackageType {
    pub fn command(&self) -> &str {
        match self {
            PackageType::Apt => "apt",
            PackageType::Chocolatey => "choco",
        }
    }

    pub fn install_command(&self) -> &str {
        match self {
            PackageType::Apt => "apt install",
            PackageType::Chocolatey => "choco install",
        }
    }
}

pub struct Package {
    /// The package manager's name.
    pub package_type: PackageType,
    /// A list of packages that are going to be installed.
    pub packages: Vec<String>,
}

impl Package {
    pub fn from_packages(package_type: PackageType, packages: Vec<String>) -> Self {
        Self {
            package_type,
            packages,
        }
    }
}

fn match_apt(line: &str) -> Result<Vec<String>, Box<dyn Error>> {
    lazy_static! {
        static ref APT_RE: Regex = Regex::new(r"apt\S*\s+install\s+(?P<name>\w+)").unwrap();
    }

    let mut packages = vec![];

    for capture in APT_RE.captures_iter(line) {
        packages.push(capture["name"].to_string());
    }

    Ok(packages)
}

pub fn catch(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    let mut packages = vec![];

    // Parse apt
    let apt_matches = match_apt(line)?;
    if apt_matches.len() > 0 {
        packages.push(Package::from_packages(PackageType::Apt, apt_matches));
    }

    Ok(packages)
}
