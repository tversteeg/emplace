use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Yay;

impl PackageManagerTrait for Yay {
    fn full_name(self) -> &'static str {
        "Yay"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["yay"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["-S"]
    }

    fn install_command(self) -> &'static str {
        "yay -S --noconfirm --quiet"
    }

    fn needs_root(self) -> bool {
        false
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("yay -Q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        // TODO
        vec![]
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
    use super::Yay;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("yay -S test").unwrap();
        assert_eq!(manager, PackageManager::from(Yay));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Yay), "sudo yay -S test" => "test");
        catch!(PackageManager::from(Yay), "yay -S test" => "test");
        catch!(PackageManager::from(Yay), "sudo yay -S lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Yay), "sudo yay -S linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Yay), "sudo yay -S test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Yay), "sudo yay test test2" => ());
    }
}
