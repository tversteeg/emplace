use ansi_term::Colour;
use run_script::ScriptOptions;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, error::Error, fmt, slice::Iter, string::String};

#[derive(Debug, Serialize, Deserialize, EnumIter)]
pub enum PackageSource {
    /// Rust cargo.
    Cargo,
    /// Debian apt-get.
    Apt,
    /// Windows chocolatey.
    Chocolatey,
    /// Python Pip
    Pip,
    /// Python Pip --user
    PipUser,
    /// Python Pip 3
    Pip3,
    /// Python Pip 3 --user
    Pip3User,
}

impl PackageSource {
    pub fn full_name(&self) -> &str {
        match self {
            PackageSource::Cargo => "Cargo Rust",
            PackageSource::Apt => "Advanced Package Tool",
            PackageSource::Chocolatey => "Chocolatey",
            PackageSource::Pip => "Python Pip",
            PackageSource::PipUser => "Python Pip --user",
            PackageSource::Pip3 => "Python Pip 3",
            PackageSource::Pip3User => "Python Pip 3 --user",
        }
    }

    pub fn colour_full_name(&self) -> String {
        Colour::Cyan
            .italic()
            .paint(format!("({})", self.full_name()))
            .to_string()
    }

    #[cfg(not(target_os = "windows"))]
    pub fn command(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo",
            PackageSource::Apt => "apt",
            PackageSource::Pip => "pip",
            PackageSource::PipUser => "pip",
            PackageSource::Pip3 => "pip3",
            PackageSource::Pip3User => "pip3",
            _ => "",
        }
    }
    #[cfg(target_os = "windows")]
    pub fn command(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo",
            PackageSource::Chocolatey => "choco",
            PackageSource::Pip => "pip",
            PackageSource::PipUser => "pip",
            PackageSource::Pip3 => "pip3",
            PackageSource::Pip3User => "pip3",
            _ => "",
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn install_command(&self) -> Vec<&str> {
        match self {
            PackageSource::Cargo => vec!["cargo", "install", "--quiet"],
            PackageSource::Apt => vec!["apt", "install"],
            PackageSource::Pip => vec!["pip", "install", "-q"],
            PackageSource::PipUser => vec!["pip", "install", "-q", "--user"],
            PackageSource::Pip3 => vec!["pip3", "install", "-q"],
            PackageSource::Pip3User => vec!["pip3", "install", "-q", "--user"],
            _ => vec![],
        }
    }
    #[cfg(target_os = "windows")]
    pub fn install_command(&self) -> Vec<&str> {
        match self {
            PackageSource::Cargo => vec!["cargo", "install", "--quiet"],
            PackageSource::Chocolatey => vec!["choco", "install", "-y"],
            PackageSource::Pip => vec!["pip", "install", "-q"],
            PackageSource::PipUser => vec!["pip", "install", "-q", "--user"],
            PackageSource::Pip3 => vec!["pip3", "install", "-q"],
            PackageSource::Pip3User => vec!["pip3", "install", "-q", "--user"],
            _ => vec![],
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_installed_script(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo install --list | grep 'v[0-9]' | grep -q",
            PackageSource::Apt => "dpkg-query --show",
            PackageSource::Pip => "pip show -q",
            PackageSource::PipUser => "pip show -q",
            PackageSource::Pip3 => "pip3 show -q",
            PackageSource::Pip3User => "pip3 show -q",
            _ => "",
        }
    }
    #[cfg(target_os = "windows")]
    pub fn is_installed_script(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo install --list | findstr",
            PackageSource::Chocolatey => "choco feature enable --name=\"'useEnhancedExitCodes'\" && choco search -le --no-color",
            PackageSource::Pip => "pip show -q",
            PackageSource::PipUser => "pip show -q",
            PackageSource::Pip3 => "pip3 show -q",
            PackageSource::Pip3User => "pip3 show -q",
            _ => ""
        }
    }

    pub fn needs_root(&self) -> bool {
        match self {
            PackageSource::Cargo => false,
            PackageSource::Apt => true,
            PackageSource::Chocolatey => true,
            PackageSource::Pip => true,
            PackageSource::PipUser => false,
            PackageSource::Pip3 => true,
            PackageSource::Pip3User => false,
        }
    }
}

impl fmt::Display for PackageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.full_name())
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
        Self { source, name }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.name, self.source.full_name())
    }

    pub fn colour_full_name(&self) -> String {
        format!(
            "{} {}",
            Colour::Yellow.paint(&*self.name),
            self.source.colour_full_name()
        )
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
        options.capture_output = true;
        options.exit_on_error = true;
        options.print_commands = false;

        let (code, _output, _error) =
            run_script::run(&*self.is_installed_script(), &vec![], &options)?;

        Ok(code == 0)
    }

    fn is_installed_script(&self) -> String {
        format!("{} {}", self.source.is_installed_script(), self.name)
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
        self.full_name() == other.full_name()
    }
}

impl Eq for Package {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages(pub Vec<Package>);

impl Packages {
    pub fn iter(&self) -> Iter<Package> {
        self.0.iter()
    }

    pub fn merge(&mut self, other: &mut Packages) {
        // Add the other packages
        self.0.append(&mut other.0);

        // Sort them so we can remove deduplicates
        self.0.sort();
        // Remove the duplicates
        self.0.dedup();
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
