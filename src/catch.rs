use regex::Regex;
use std::{
    fmt,
    error::Error,
    string::String,
};

pub enum PackageSource {
    /// Rust cargo.
    Cargo,
    /// Debian apt-get.
    Apt,
    /// Windows chocolatey.
    Chocolatey,
}

impl PackageSource {
    pub fn command(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo",
            PackageSource::Apt => "apt",
            PackageSource::Chocolatey => "choco",
        }
    }

    pub fn install_command(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo install",
            PackageSource::Apt => "apt install",
            PackageSource::Chocolatey => "choco install",
        }
    }

    pub fn full_name(&self) -> &str {
        match self {
            PackageSource::Cargo => "Cargo Rust",
            PackageSource::Apt => "Advanced Package Tool",
            PackageSource::Chocolatey => "Chocolatey",
        }
    }
}

impl fmt::Display for PackageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

pub struct Package {
    /// The package manager's name.
    pub package_source: PackageSource,
    /// A list of packages that are going to be installed.
    pub install_name: String,
}

impl Package {
    pub fn new(package_source: PackageSource, install_name: String) -> Self {
        Self {
            package_source,
            install_name,
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.install_name, self.package_source)
    }
}

fn match_cargo(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref CARGO_RE: Regex = Regex::new(r"cargo\s+install\s+(?P<name>\w+)+").unwrap();
    }

    let mut packages = vec![];

    for capture in CARGO_RE.captures_iter(line) {
        packages.push(Package::new(PackageSource::Cargo, capture["name"].to_string()));
    }

    Ok(packages)
}

fn match_apt(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref APT_RE: Regex = Regex::new(r"apt\S*\s+install\s+(?P<name>\w+)+").unwrap();
    }

    let mut packages = vec![];

    for capture in APT_RE.captures_iter(line) {
        packages.push(Package::new(PackageSource::Apt, capture["name"].to_string()));
    }

    Ok(packages)
}

fn match_choco(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    lazy_static! {
        static ref CHOCO_RE: Regex = Regex::new(r"choco\S*\s+install\s+(?P<name>\w+)+").unwrap();
    }

    let mut packages = vec![];

    for capture in CHOCO_RE.captures_iter(line) {
        packages.push(Package::new(PackageSource::Chocolatey, capture["name"].to_string()));
    }

    Ok(packages)
}

pub fn catch(line: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    let mut packages = vec![];

    // Parse Cargo
    packages.append(&mut match_cargo(line)?);

    // Parse APT
    packages.append(&mut match_apt(line)?);

    // Parse Chocolatey
    packages.append(&mut match_choco(line)?);

    Ok(packages)
}
