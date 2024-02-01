use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rustup;

impl PackageManagerTrait for Rustup {
    fn full_name(self) -> &'static str {
        "Rustup Rust"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["rustup"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["component add"]
    }

    fn install_command(self) -> &'static str {
        "rustup component add"
    }

    fn needs_root(self) -> bool {
        false
    }

    #[cfg(not(target_os = "windows"))]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("rustup component list | grep -q {}", package))
    }
    #[cfg(target_os = "windows")]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("rustup component list | findstr {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec![]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![
            CaptureFlag::DynamicValue("--target"),
            CaptureFlag::DynamicValue("--toolchain"),
        ]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Rustup;
    use crate::package_manager::PackageManager;

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("rustup component add test").unwrap();
        assert_eq!(manager, PackageManager::from(Rustup));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Rustup), "rustup component add test" => "test");

        // Multiple
        catch!(PackageManager::from(Rustup), "rustup component add test test2" => "test", "test2");
    }
}
