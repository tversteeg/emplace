use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Cargo;

impl PackageManagerTrait for Cargo {
    fn full_name(self) -> &'static str {
        "Cargo Rust"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["cargo"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "cargo install --quiet"
    }

    fn needs_root(self) -> bool {
        false
    }

    #[cfg(not(target_os = "windows"))]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!(
            "cargo install --list | grep 'v[0-9]' | grep -q {}",
            package
        ))
    }
    #[cfg(target_os = "windows")]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("cargo install --list | findstr {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec!["-Z", "--version", "-j", "--jobs"]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![
            CaptureFlag::Single("--git"),
            CaptureFlag::DynamicValue("--branch"),
            CaptureFlag::Single("+nightly"),
            CaptureFlag::Single("+stable"),
            CaptureFlag::Single("+beta"),
            CaptureFlag::Single("--no-default-features"),
            CaptureFlag::DynamicValue("--features"),
        ]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec!["--path"]
    }
}

#[cfg(test)]
mod tests {
    use super::Cargo;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        assert!(PackageManager::from_line_iter("cargo binstall test")
            .any(|manager| manager == PackageManager::from(Cargo)));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Cargo), "cargo install test" => "test");

        // Multiple
        catch!(PackageManager::from(Cargo), "cargo install test test2" => "test", "test2");
        catch!(PackageManager::from(Cargo), "cargo install test install test-install" => "test", "install", "test-install");

        // Shouldn't match
        catch!(PackageManager::from(Cargo), "cargo uninstall test test2" => ());
        catch!(PackageManager::from(Cargo), "cargo install ." => ());
        catch!(PackageManager::from(Cargo), "cargo install --path test" => ());
        catch!(PackageManager::from(Cargo), "cargo binstall test" => ());

        // Flags that should be captured
        catch!(PackageManager::from(Cargo), "cargo install --git https://test.com/test.git" => "https://test.com/test.git" ["--git"]);
        catch!(PackageManager::from(Cargo), "cargo install --git https://test.com/test.git --branch 3.x" => "https://test.com/test.git" ["--git", "--branch 3.x"]);
        catch!(PackageManager::from(Cargo), "cargo install +nightly test" => "test" ["+nightly"]);
    }
}
