use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Chocolatey;

impl PackageManagerTrait for Chocolatey {
    fn full_name(self) -> &'static str {
        "Chocolatey"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["choco"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "choco install -y"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!(
            "choco feature enable --name=\"'useEnhancedExitCodes'\" && choco search -le --no-color {}",
            package
        ))
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
    use super::Chocolatey;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("choco install test").unwrap();
        assert_eq!(manager, PackageManager::from(Chocolatey));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Chocolatey), "choco install test" => "test");

        // Multiple
        catch!(PackageManager::from(Chocolatey), "choco install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Chocolatey), "choco test test2" => ());
    }
}
