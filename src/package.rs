use serde::{Serialize, Deserialize};
use std::{
    fmt,
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
            1 => format!("Emplace - mirror package \"{}\"", self.0[0]),
            n => format!("Emplace - mirror {} packages", n),
        }
    }
}
