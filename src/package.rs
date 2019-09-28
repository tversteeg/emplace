use ansi_term::Colour;
use serde::{Serialize, Deserialize};
use std::{
    fmt,
    string::String,
    slice::Iter
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
    pub fn full_name(&self) -> &str {
        match self {
            PackageSource::Cargo => "Cargo Rust",
            PackageSource::Apt => "Advanced Package Tool",
            PackageSource::Chocolatey => "Chocolatey",
        }
    }

#[cfg(not(target_os = "windows"))]
    pub fn command(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo",
            PackageSource::Apt => "apt",
            _ => ""
        }
    }
#[cfg(target_os = "windows")]
    pub fn command(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo",
            PackageSource::Chocolatey => "choco",
            _ => ""
        }
    }

#[cfg(not(target_os = "windows"))]
    pub fn install_command(&self) -> Vec<&str> {
        match self {
            PackageSource::Cargo => vec!["cargo", "install"],
            PackageSource::Apt => vec!["apt", "install"],
            _ => vec![]
        }
    }
#[cfg(target_os = "windows")]
    pub fn install_command(&self) -> Vec<&str> {
        match self {
            PackageSource::Cargo => vec!["cargo", "install"],
            PackageSource::Chocolatey => vec!["choco", "install", "-y"],
            _ => vec![]
        }
    }

#[cfg(not(target_os = "windows"))]
    pub fn is_installed_command(&self) -> Vec<&str> {
        match self {
            PackageSource::Cargo => vec!["sh", "-c", "\"cargo --list | grep -q\""],
            PackageSource::Apt => vec!["dpkg", "-s"],
            _ => vec![]
        }
    }
#[cfg(target_os = "windows")]
    pub fn is_installed_command(&self) -> Vec<&str> {
        match self {
            PackageSource::Cargo => vec!["cmd", "/k",
                "cargo --list | findstr"],
            PackageSource::Chocolatey => vec!["cmd", "/k",
                "choco feature enable --name=\"'useEnhancedExitCodes'\" && choco search -le --no-color"],
            _ => vec![]
        }
    }
}

impl fmt::Display for PackageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Colour::Cyan.italic().paint(format!("({})", self.full_name())))
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

    pub fn command(&self) -> &str {
        self.source.command()
    }

    pub fn install_command(&self) -> Vec<&str> {
        let mut commands = self.source.install_command();
        commands.push(&*self.name);

        commands
    }

    pub fn is_installed_command(&self) -> Vec<&str> {
        let mut commands = self.source.is_installed_command();
        commands.push(&*self.name);

        commands
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", Colour::Yellow.paint(&self.name), self.source)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages(pub Vec<Package>);

impl Packages {
    pub fn iter(&self) -> Iter<Package> {
        self.0.iter()
    }

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
