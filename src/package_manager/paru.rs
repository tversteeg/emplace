use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Paru;

impl PackageManagerTrait for Paru {
    fn full_name(self) -> &'static str {
        "Paru"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["paru"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["-S"]
    }

    fn install_command(self) -> &'static str {
        "paru -S --noconfirm --quiet"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("paru -Q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec![
            // inherited from pacman
            "-b",
            "--dbpath",
            "-r",
            "--root",
            "--arch",
            "--cachedir",
            "--color",
            "--config",
            "--gpgdir",
            "--hookdir",
            "--logfile",
            "--sysroot",
            "--assume-installed",
            "--print-format",
            "--ignore",
            "--ignoregroup",
            "--overwrite",
            "-o",
            "--owns",
            "-s",
            "--search",
            "--asdeps",
            "--asexplicit",
            // new in paru
            "--clonedir",
            "--makepkg",
            "--makepkgconf",
            "--pacman",
            "--pacman-conf",
            "--git",
            "--gitflags",
            "--gpg",
            "--gpgflags",
            "--fm",
            "--asp",
            "--mflags",
            "--bat",
            "--batflags",
            "--sudo",
            "--sudoflags",
            "--chrootflags",
            "--completioninterval",
            "--sortby",
            "--searchby",
            "--removemake",
            "--limit",
            "--redownload",
            "--rebuild",
            "--sudoloop",
            "--localrepo",
            "--chroot",
            "--sign",
            "--signdb",
        ]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Paru;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("paru -S test").unwrap();
        assert_eq!(manager, PackageManager::from(Paru));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Paru), "sudo paru -S test" => "test");
        catch!(PackageManager::from(Paru), "paru -S test" => "test");
        catch!(PackageManager::from(Paru), "sudo paru -S lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Paru), "sudo paru -S linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Paru), "sudo paru -S test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Paru), "sudo paru test test2" => ());
    }
}
