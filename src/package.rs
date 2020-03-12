use anyhow::Context;
use colored::*;
use itertools::Itertools;
use run_script::ScriptOptions;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    error::Error,
    fmt,
    slice::{Iter, IterMut},
    string::String,
};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, EnumIter)]
pub enum PackageSource {
    /// Rust cargo
    Cargo,
    /// Rust rustup component
    RustupComponent,
    /// Debian apt-get
    Apt,
    /// Arch Pacman
    Pacman,
    /// Linux Snap
    Snap,
    /// Windows chocolatey
    Chocolatey,
    /// Windows scoop
    Scoop,
    /// Python Pip
    Pip,
    /// Python Pip --user
    PipUser,
    /// Python Pip 3
    Pip3,
    /// Python Pip 3 --user
    Pip3User,
    /// Node Package Manager
    Npm,
}

impl PackageSource {
    pub fn full_name(&self) -> &str {
        match self {
            PackageSource::Cargo => "Cargo Rust",
            PackageSource::RustupComponent => "Rustup Component",
            PackageSource::Apt => "Advanced Package Tool",
            PackageSource::Pacman => "Pacman",
            PackageSource::Snap => "Snap",
            PackageSource::Chocolatey => "Chocolatey",
            PackageSource::Scoop => "Scoop",
            PackageSource::Pip => "Python Pip",
            PackageSource::PipUser => "Python Pip --user",
            PackageSource::Pip3 => "Python Pip 3",
            PackageSource::Pip3User => "Python Pip 3 --user",
            PackageSource::Npm => "Node Package Manager",
        }
    }

    pub fn colour_full_name(self) -> String {
        format!("({})", self.full_name())
            .cyan()
            .italic()
            .to_string()
    }

    #[cfg(not(target_os = "windows"))]
    pub fn command<'a>(self) -> &'a str {
        match self {
            PackageSource::Cargo => "cargo",
            PackageSource::RustupComponent => "rustup",
            PackageSource::Apt => "apt",
            PackageSource::Pacman => "pacman",
            PackageSource::Snap => "snap",
            PackageSource::Pip => "pip",
            PackageSource::PipUser => "pip",
            PackageSource::Pip3 => "pip3",
            PackageSource::Pip3User => "pip3",
            PackageSource::Npm => "npm",
            _ => "",
        }
    }
    #[cfg(target_os = "windows")]
    pub fn command<'a>(self) -> &'a str {
        match self {
            PackageSource::Cargo => "cargo",
            PackageSource::RustupComponent => "rustup",
            PackageSource::Chocolatey => "choco",
            PackageSource::Scoop => "scoop",
            PackageSource::Pip => "pip",
            PackageSource::PipUser => "pip",
            PackageSource::Pip3 => "pip3",
            PackageSource::Pip3User => "pip3",
            PackageSource::Npm => "npm",
            _ => "",
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn install_command<'a>(self) -> Vec<&'a str> {
        match self {
            PackageSource::Cargo => vec!["cargo", "install", "--quiet"],
            PackageSource::RustupComponent => vec!["rustup", "component", "add"],
            PackageSource::Apt => vec!["apt", "install"],
            PackageSource::Pacman => vec!["pacman", "-Sy", "--noconfirm", "--quiet"],
            PackageSource::Snap => vec!["snap", "install"],
            PackageSource::Pip => vec!["pip", "install", "-q"],
            PackageSource::PipUser => vec!["pip", "install", "-q", "--user"],
            PackageSource::Pip3 => vec!["pip3", "install", "-q"],
            PackageSource::Pip3User => vec!["pip3", "install", "-q", "--user"],
            PackageSource::Npm => vec!["npm", "install", "-g"],
            _ => vec![],
        }
    }
    #[cfg(target_os = "windows")]
    pub fn install_command<'a>(self) -> Vec<&'a str> {
        match self {
            PackageSource::Cargo => vec!["cargo", "install", "--quiet"],
            PackageSource::RustupComponent => vec!["rustup", "component", "add"],
            PackageSource::Chocolatey => vec!["choco", "install", "-y"],
            PackageSource::Scoop => vec!["scoop", "install"],
            PackageSource::Pip => vec!["pip", "install", "-q"],
            PackageSource::PipUser => vec!["pip", "install", "-q", "--user"],
            PackageSource::Pip3 => vec!["pip3", "install", "-q"],
            PackageSource::Pip3User => vec!["pip3", "install", "-q", "--user"],
            PackageSource::Npm => vec!["npm", "install", "-g"],
            _ => vec![],
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_installed_script<'a>(self) -> Option<&'a str> {
        match self {
            PackageSource::Cargo => Some("cargo install --list | grep 'v[0-9]' | grep -q"),
            PackageSource::RustupComponent => Some("rustup component list | grep -q"),
            PackageSource::Apt => Some("dpkg-query --show"),
            PackageSource::Pacman => Some("pacman -Q"),
            PackageSource::Snap => Some("snap | grep -Eo '^[^ ]+' | grep -q"),
            PackageSource::Pip => Some("pip show -q"),
            PackageSource::PipUser => Some("pip show -q"),
            PackageSource::Pip3 => Some("pip3 show -q"),
            PackageSource::Pip3User => Some("pip3 show -q"),
            PackageSource::Npm => Some("npm list --depth=0 -g | grep -q"),
            _ => None,
        }
    }
    #[cfg(target_os = "windows")]
    pub fn is_installed_script<'a>(self) -> Option<&'a str> {
        match self {
            PackageSource::Cargo => Some("cargo install --list | findstr"),
            PackageSource::RustupComponent => Some("rustup component list | findstr"),
            PackageSource::Chocolatey => Some("choco feature enable --name=\"'useEnhancedExitCodes'\" && choco search -le --no-color"),
            PackageSource::Scoop => Some("scoop list | findstr"),
            PackageSource::Pip => Some("pip show -q"),
            PackageSource::PipUser => Some("pip show -q"),
            PackageSource::Pip3 => Some("pip3 show -q"),
            PackageSource::Pip3User => Some("pip3 show -q"),
            PackageSource::Npm => Some("npm list --depth=0 -g | findstr"),
            _ => None
        }
    }

    pub fn needs_root(self) -> bool {
        match self {
            PackageSource::Cargo => false,
            PackageSource::RustupComponent => false,
            PackageSource::Apt => true,
            PackageSource::Pacman => true,
            PackageSource::Snap => true,
            PackageSource::Chocolatey => true,
            PackageSource::Scoop => true,
            PackageSource::Pip => true,
            PackageSource::PipUser => false,
            PackageSource::Pip3 => true,
            PackageSource::Pip3User => false,
            PackageSource::Npm => false,
        }
    }
}

impl fmt::Display for PackageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.full_name())
    }
}

impl PartialEq for PackageSource {
    fn eq(&self, other: &Self) -> bool {
        self.full_name() == other.full_name()
    }
}

impl Eq for PackageSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// The package manager's name.
    pub source: PackageSource,
    /// A list of packages that are going to be installed.
    pub name: String,
    /// The name without the flags.
    #[serde(skip)]
    package_name: Option<String>,
}

impl Package {
    pub fn new(source: PackageSource, name: String) -> Self {
        let mut package = Self {
            source,
            name,
            package_name: None,
        };

        package.set_package_name();

        package
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.name, self.source.full_name())
    }

    pub fn colour_full_name(&self) -> String {
        format!("{} {}", self.name.yellow(), self.source.colour_full_name())
    }

    pub fn command(&self) -> &str {
        self.source.command()
    }

    pub fn install_command(&self) -> Vec<&str> {
        let mut commands = self.source.install_command();
        if self.source.needs_root() {
            commands.insert(0, "sudo");
        }
        commands.push(&*self.name);

        commands
    }

    pub fn is_installed(&self) -> Result<bool, Box<dyn Error>> {
        let mut options = ScriptOptions::new();
        options.exit_on_error = true;
        options.print_commands = false;

        let install_script = match self.is_installed_script() {
            Some(install_script) => install_script,
            // Return that it's installed when this package is not available on this OS
            None => return Ok(true),
        };

        let (code, _output, _error) = run_script::run(&*install_script, &vec![], &options)
            .context("could not run is installed script")?;

        Ok(code == 0)
    }

    pub fn set_package_name(&mut self) {
        let package_name = self
            .name
            .split(' ')
            // If the string starts with - ignore it, so all the flags
            .filter(|s| !s.starts_with('-'))
            // If the string starts with http get the last part which is a gamble but most of the
            // times it matches the package name
            .map(|s| {
                if s.starts_with("http") {
                    s.split('/').last().unwrap()
                } else {
                    s
                }
            })
            .join(" ");

        self.package_name = Some(package_name);
    }

    fn is_installed_script(&self) -> Option<String> {
        let package_name = match &self.package_name {
            Some(package_name) => package_name,
            None => {
                panic!("Could not get package name, function `set_package_name` not called yet")
            }
        };

        match self.source.is_installed_script() {
            Some(script) => Some(format!("{} {}", script, package_name)),
            None => None,
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_name().cmp(&other.full_name())
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.full_name() == other.full_name() && self.source == other.source
    }
}

impl Eq for Package {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages(pub Vec<Package>);

impl Packages {
    pub fn iter(&self) -> Iter<Package> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Package> {
        self.0.iter_mut()
    }

    pub fn merge(&mut self, other: &mut Packages) {
        // Add the other packages
        self.0.append(&mut other.0);

        // Sort them so we can remove deduplicates
        self.0.sort();
        // Remove the duplicates
        self.0.dedup();
    }

    pub fn filter_saved_packages(&mut self, old: &Packages) {
        self.0 = self
            .0
            .iter()
            .filter(|package| !old.iter().any(|old_package| *package == old_package))
            .cloned()
            .collect();
    }

    pub fn commit_message(&self) -> String {
        match self.0.len() {
            0 => panic!("Can't create a commit message for empty changes"),
            1 => format!("Emplace - mirror package \"{}\"", self.0[0]),
            n => format!("Emplace - mirror {} packages", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packages_deduplication() {
        let package = Package::new(PackageSource::Cargo, "test".to_string());
        let duplicate_package = Package::new(PackageSource::Cargo, "test".to_string());

        let packages_vec = vec![package];
        let duplicate_packages_vec = vec![duplicate_package];

        let mut packages = Packages(packages_vec);
        let mut duplicate_packages = Packages(duplicate_packages_vec);

        packages.merge(&mut duplicate_packages);
        assert_eq!(1, packages.0.len());
    }
}
