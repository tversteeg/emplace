use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Zypper;

impl PackageManagerTrait for Zypper {
    fn full_name(self) -> &'static str {
        "Command-line interface to ZYpp system management library"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["zypper", "zin", "zinr"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install", "in", "inr"]
    }

    fn install_command(self) -> &'static str {
        "zypper install -y"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("zypper info {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec!["-c", "--config", "-q", "--quiet"]
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
        catch!(PackageManager::from(Zypper), "zypper install test" => "test");
        catch!(PackageManager::from(Zypper), "sudo zypper install libgfortran5-32bit" => "libgfortran5-32bit");
        catch!(PackageManager::from(Zypper), "sudo zypper install perf" => "perf");

        // Multiple
        catch!(PackageManager::from(Zypper), "sudo zypper install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Zypper), "sudo zypper test test2" => ());
        catch!(PackageManager::from(Zypper), "sudo zypper install -f" => ());
    }
}
