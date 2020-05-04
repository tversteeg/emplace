use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Scoop;

impl PackageManagerTrait for Scoop {
    fn full_name(self) -> &'static str {
        "Scoop"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["scoop"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "scoop install"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("scoop list | findstr {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        // TODO
        vec![]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Scoop;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::from_line("scoop install test").unwrap();
        assert_eq!(manager, PackageManager::from(Scoop));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Scoop), "scoop install test" => "test");

        // Multiple
        catch!(PackageManager::from(Scoop), "scoop install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Scoop), "scoop test test2" => ());
    }
}
