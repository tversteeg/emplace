use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pacman;

impl PackageManagerTrait for Pacman {
    fn full_name(self) -> &'static str {
        "Pacman"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["pacman"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["-S"]
    }

    fn install_command(self) -> &'static str {
        "pacman -S --noconfirm --quiet"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("pacman -Q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        // Source: https://archlinux.org/pacman/pacman.8.html
        vec![
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
    use super::Pacman;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("pacman -S test").unwrap();
        assert_eq!(manager, PackageManager::from(Pacman));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Pacman), "sudo pacman -S test" => "test");
        catch!(PackageManager::from(Pacman), "pacman -S test" => "test");
        catch!(PackageManager::from(Pacman), "sudo pacman -S lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Pacman), "sudo pacman -S linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Pacman), "sudo pacman -S test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Pacman), "sudo pacman test test2" => ());
    }
}
