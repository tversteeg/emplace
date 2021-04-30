#[macro_use]
pub mod test_macro;

mod apt;
mod brew;
mod cargo;
mod chocolatey;
mod dnf;
mod gem;
mod go;
mod guix;
mod nix;
mod npm;
mod pacman;
mod pip;
mod pip3;
mod pkg;
mod rua;
mod rustup;
mod scoop;
mod snap;
mod yay;
mod zypper;

pub use apt::Apt;
pub use brew::Brew;
pub use cargo::Cargo;
pub use chocolatey::Chocolatey;
pub use dnf::Dnf;
pub use gem::Gem;
pub use go::Go;
pub use guix::Guix;
pub use nix::Nix;
pub use npm::Npm;
pub use pacman::Pacman;
pub use pip::Pip;
pub use pip3::Pip3;
pub use pkg::Pkg;
pub use rua::Rua;
pub use rustup::Rustup;
pub use scoop::Scoop;
pub use snap::Snap;
pub use yay::Yay;
pub use zypper::Zypper;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::EnumIter;

/// Enum containing all package managers.
///
/// The actual functions are implemented in `src/package_manager_impl.rs`.
#[enum_dispatch::enum_dispatch]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum PackageManager {
    Apt,
    Brew,
    Cargo,
    Chocolatey,
    Dnf,
    Gem,
    Go,
    Guix,
    Nix,
    Npm,
    Pacman,
    Pip,
    Pip3,
    Pkg,
    Rua,
    Rustup,
    Scoop,
    Snap,
    Yay,
    Zypper,
}

/// Trait that needs to be implemented for a new package manager.
#[enum_dispatch::enum_dispatch(PackageManager)]
pub trait PackageManagerTrait {
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
    fn capture_flags(self) -> Vec<CaptureFlag>;

    /// Flags that make a command with a subcommand that would be caught otherwise be ignored.
    fn invalidating_flags(self) -> Vec<&'static str>;
}

/// Different ways in which a set of flags can be captured.
#[derive(Debug, Copy, Clone)]
pub enum CaptureFlag {
    /// A single flag argument without a value.
    Single(&'static str),
    /// A flag argument with a set value.
    SetValue(&'static str, &'static str),
    /// A flag argument with a single dynamic value.
    DynamicValue(&'static str),
}

/// The way a package is checked if it's installed.
pub enum PackageInstalledMethod {
    /// A simple command line script will check if the package is installed.
    Script(String),

    // TODO use this
    #[allow(unused)]
    /// A file or directory on the filesystem is checked for existence.
    Path(PathBuf),
}

#[cfg(test)]
mod tests {
    use crate::{
        catch,
        package_manager::{apt::Apt, PackageManager},
    };

    #[test]
    fn test_empty() {
        catch!(PackageManager::from(Apt), "no match" => ());
    }
}
