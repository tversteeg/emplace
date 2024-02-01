use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dnf;

impl PackageManagerTrait for Dnf {
    fn full_name(self) -> &'static str {
        "Dandified yum"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["dnf", "yum"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "dnf install -y"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("dnf info {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        // TODO
        vec![]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        // TODO
        vec![]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Dnf;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("dnf install test").unwrap();
        assert_eq!(manager, PackageManager::from(Dnf));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Dnf), "sudo dnf install test" => "test");
        catch!(PackageManager::from(Dnf), "dnf install test" => "test");
        catch!(PackageManager::from(Dnf), "sudo yum install test" => "test");
        catch!(PackageManager::from(Dnf), "sudo dnf install lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Dnf), "sudo dnf install linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Dnf), "sudo dnf install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Dnf), "sudo dnf test test2" => ());
        catch!(PackageManager::from(Dnf), "sudo dnf install -f" => ());
    }
}
