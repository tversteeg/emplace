use ansi_term::Colour;
use serde::{Serialize, Deserialize};
use run_script::ScriptOptions;
use std::{
    fmt,
    string::String,
    slice::Iter,
    error::Error,
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

    pub fn colour_full_name(&self) -> String {
        Colour::Cyan.italic().paint(format!("({})", self.full_name())).to_string()
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
            PackageSource::Cargo => vec!["cargo", "install", "--quiet"],
            PackageSource::Apt => vec!["apt", "install"],
            _ => vec![]
        }
    }
#[cfg(target_os = "windows")]
    pub fn install_command(&self) -> Vec<&str> {
        match self {
            PackageSource::Cargo => vec!["cargo", "install", "--quiet"],
            PackageSource::Chocolatey => vec!["choco", "install", "-y"],
            _ => vec![]
        }
    }

#[cfg(not(target_os = "windows"))]
    pub fn is_installed_script(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo install --list | grep 'v[0-9]' | grep -q",
            PackageSource::Apt => "dpkg-query --show",
            _ => ""
        }
    }
#[cfg(target_os = "windows")]
    pub fn is_installed_script(&self) -> &str {
        match self {
            PackageSource::Cargo => "cargo --list | findstr",
            PackageSource::Chocolatey => "choco feature enable --name=\"'useEnhancedExitCodes'\" && choco search -le --no-color",
            _ => ""
        }
    }

    pub fn needs_root(&self) -> bool {
        match self {
            PackageSource::Cargo => false,
            PackageSource::Apt => true,
            PackageSource::Chocolatey => true,
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
        Self {
            source,
            name,
        }
    }

    pub fn colour_name(&self) -> String {
        format!("{} {}", Colour::Yellow.paint(&*self.name), self.source.colour_full_name())
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

        let (code, _output, _error) = run_script::run(&*self.is_installed_script(), &vec![], &options)?;

        Ok(code == 0)
    }

    fn is_installed_script(&self) -> String {
        format!("{} {}", self.source.is_installed_script(), self.name)
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.source)
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
