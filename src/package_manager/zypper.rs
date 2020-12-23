use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Zypper;

impl PackageManagerTrait for Zypper {
    fn full_name(self) -> &'static str {
        "Zypper"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["zypper"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["in", "install"]
    }

    fn install_command(self) -> &'static str {
        "zypper install -y"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("rpm -q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        // TODO
        vec![]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        // TODO
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Zypper;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::from_line("zypper install test").unwrap();
        assert_eq!(manager, PackageManager::from(Zypper));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Zypper), "sudo zypper install test" => "test");
        catch!(PackageManager::from(Zypper), "sudo zypper in test" => "test");
        catch!(PackageManager::from(Zypper), "zypper install test" => "test");
        catch!(PackageManager::from(Zypper), "zypper in test" => "test");
        catch!(PackageManager::from(Zypper), "sudo zypper install lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Zypper), "sudo zypper in lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Zypper), "sudo zypper install linux-perf-5.3" => "linux-perf-5.3");
        catch!(PackageManager::from(Zypper), "sudo zypper in linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Zypper), "sudo zypper install test test2" => "test", "test2");
        catch!(PackageManager::from(Zypper), "sudo zypper in test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Zypper), "sudo zypper test test2" => ());
    }
}
