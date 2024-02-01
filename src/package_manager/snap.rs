use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snap;

impl PackageManagerTrait for Snap {
    fn full_name(self) -> &'static str {
        "Snap"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["snap"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "snap install"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("snap | grep -Eo '^[^ ]+' | grep -q {}", package))
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
    use super::Snap;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("snap install test").unwrap();
        assert_eq!(manager, PackageManager::from(Snap));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Snap), "sudo snap install test" => "test");
        catch!(PackageManager::from(Snap), "snap install test" => "test");
        catch!(PackageManager::from(Snap), "sudo snap install lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Snap), "sudo snap install linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Snap), "sudo snap install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Snap), "sudo snap test test2" => ());
    }
}
