mod apt;

#[macro_use]
pub mod test_macro;

use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};
use strum_macros::EnumIter;

/// Enum containing all package managers.
///
/// The actual functions are implemented in `src/package_manager_impl.rs`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum PackageManager {
    Apt,
    Cargo,
    RustupComponent,
    Pacman,
    Rua,
    Snap,
    Chocolatey,
    Scoop,
    Pip,
    Pip3,
    Npm,
    Yay,
    Nix,

    // Deprecated
    PipUser,
    Pip3User,
}

impl PackageManager {
    /// Return the package manager trait object depending on the enum variant.
    pub fn get(self) -> impl PackageManagerTrait {
        match self {
            PackageManager::Apt => apt::Apt,
            _ => todo!("package manager is not implemented yet"),
        }
    }
}

/// Trait that needs to be implemented for a new package manager.
pub trait PackageManagerTrait:
    fmt::Debug + Default + Copy + Clone + Eq + PartialEq + Sized
{
    /// A descriptive name.
    fn full_name(self) -> &'static str;

    /// The command-line word(s) to invoke the package manager.
    fn commands(self) -> Vec<&'static str>;

    /// The command-line subcommand(s) that's used to catch installing new packages.
    fn sub_commands(self) -> Vec<&'static str>;

    /// Command that's used to install new packages.
    fn install_command(self) -> &'static str;

    /// Does installing a new command need administrative permissions?
    fn needs_root(self) -> bool;

    /// Check whether a package is already installed.
    fn is_installed(self, package: &str) -> PackageInstalledMethod;

    /// A list of known command line flags that accept an extra argument which could be the name of
    /// the package.
    fn known_flags_with_values(self) -> Vec<&'static str>;

    /// A list of command line flags that should be caught as well.
    ///
    /// This usually stays empty.
    /// A list of tuples is returned where the tuple contains the flag arguments.
    /// For example `-t experimental` is supplied like this:
    ///
    /// ```rust
    /// # struct Something;
    /// # impl Something {
    /// fn capture_flags(self) -> Vec<(&'static str, Option<&'static str>)> {
    ///     vec![("-t", Some("experimental"))]
    /// }
    /// # }
    /// ```
    fn capture_flags(self) -> Vec<(&'static str, Option<&'static str>)> {
        vec![]
    }
}

/// The way a package is checked if it's installed.
pub enum PackageInstalledMethod {
    /// A simple command line script will check if the package is installed.
    Script(String),
    /// A file or directory on the filesystem is checked for existence.
    Path(PathBuf),
}
