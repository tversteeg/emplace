use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pkg;

impl PackageManagerTrait for Pkg {
    fn full_name(self) -> &'static str {
        "Pkg"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["pkg"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "pkg install -y"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("pkg_info | grep -q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec![]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![
            CaptureFlag::DynamicValue("--repository"),
            CaptureFlag::DynamicValue("-r"),
        ]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Pkg;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("pkg install test").unwrap();
        assert_eq!(manager, PackageManager::from(Pkg));

        assert!(PackageManager::single_from_line("qpkg install test").is_none());
    }

    #[test]
    fn test_catch() {
        catch!(PackageManager::from(Pkg), "pkg install test" => "test");
    }
}
