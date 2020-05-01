mod apt;

use crate::command::call;
use std::{fmt, fs, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Enum containing all package managers.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum PackageManager {
    Apt(apt::Apt),
}

impl PackageManager {
    /// Try to find the proper package manager corresponding to a line.
    pub fn from_line(line: &str) -> Option<Self> {
        Self::iter().find(|manager| line.contains(manager.command()))
    }

    /// A descriptive name.
    pub fn full_name(self) -> &'static str {
        match self {
            manager => manager.full_name(),
        }
    }

    /// The command-line word to invoke the package manager.
    pub fn command(self) -> &'static str {
        match self {
            manager => manager.command(),
        }
    }

    /// Command that's used to install new packages.
    pub fn install_command(self) -> &'static str {
        match self {
            manager => manager.install_command(),
        }
    }

    /// Does installing a new command need administrative permissions?
    pub fn needs_root(self) -> bool {
        match self {
            manager => manager.needs_root(),
        }
    }

    /// Check whether a package is already installed.
    pub fn is_installed(self, package: &str) -> bool {
        let method = match self {
            manager => manager.is_installed(package),
        };

        match method {
            PackageInstalledMethod::Script(script) => {}
            PackageInstalledMethod::Path(path) => fs::exists(path),
        }
    }
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.full_name())
    }
}

/// Trait that needs to be implemented for a new package manager.
pub trait PackageManagerTrait:
    fmt::Debug + Default + Copy + Clone + Eq + PartialEq + Sized
{
    /// A descriptive name.
    fn full_name() -> &'static str;

    /// The command-line word to invoke the package manager.
    fn command() -> &'static str;

    /// Command that's used to install new packages.
    fn install_command() -> &'static str;

    /// Does installing a new command need administrative permissions?
    fn needs_root() -> bool;

    /// Check whether a package is already installed.
    fn is_installed(package: &str) -> PackageInstalledMethod;
}

/// The way a package is checked if it's installed.
pub enum PackageInstalledMethod {
    /// A simple command line script will check if the package is installed.
    Script(String),
    /// A file or directory on the filesystem is checked for existence.
    Path(PathBuf),
}
