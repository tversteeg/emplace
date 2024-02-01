use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Npm;

impl PackageManagerTrait for Npm {
    fn full_name(self) -> &'static str {
        "Node Package Manager"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["npm"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "npm install -g"
    }

    fn needs_root(self) -> bool {
        false
    }

    #[cfg(not(target_os = "windows"))]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("npm list --depth=0 -g | grep -q {}", package))
    }
    #[cfg(target_os = "windows")]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("npm list --depth=0 -g | findstr {}", package))
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
    use super::Npm;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("npm install test").unwrap();
        assert_eq!(manager, PackageManager::from(Npm));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Npm), "npm install -g test" => "test");
        catch!(PackageManager::from(Npm), "npm --global install test" => "test");

        // Ignore
        // TODO
        //catch!(PackageManager::from(Npm), "npm install test" => ());
    }
}
