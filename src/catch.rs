use serde::{Serialize, Deserialize};
use regex::Regex;
use std::{
    fmt,
    error::Error,
    string::String,
};

#[derive(Debug, Serialize, Deserialize, EnumIter)]
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

    pub fn is_installed_command(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo install",
            PackageSource::Apt => "dpkg -s",
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    /// The package manager's name.
    pub source: PackageSource,
    /// A list of packages that are going to be installed.
    pub name: String,
}

impl Package {
    pub fn new(source: PackageSource, name: String) -> Self {
        Self {
            source,
            name,
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.source)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages(pub Vec<Package>);

impl Packages {
    pub fn merge(&mut self, other: &mut Packages) {
        self.0.append(&mut other.0);
    }

    pub fn commit_message(&self) -> String {
        match self.0.len() {
            0 => panic!("Can't create a commit message for empty changes"),
            1 => format!("Added a new package ({})", self.0[0]),
            n => format!("Added {} new packages", n),
        }
    }
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
