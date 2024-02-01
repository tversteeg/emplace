use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CargoBinstall;

impl PackageManagerTrait for CargoBinstall {
    fn full_name(self) -> &'static str {
        "Cargo B(inary)Install"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["cargo"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["binstall"]
    }

    fn install_command(self) -> &'static str {
        "cargo binstall --quiet"
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
        vec![
            "--log-level",
            "--github-token",
            "--root-certificates",
            "--min-tls-version",
            "--registry",
            "--index",
            "--root",
            "--install-path",
            "--disable-strategies",
            "--strategies",
            "--rate-limit",
            "--pkg-url",
            "--pkg-fmt",
            "--bin-dir",
            "--manifest-path",
        ]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![
            CaptureFlag::Single("--git"),
            CaptureFlag::DynamicValue("--version"),
            CaptureFlag::DynamicValue("--targets"),
        ]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::CargoBinstall;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        assert!(PackageManager::from_line_iter("cargo binstall test")
            .any(|manager| manager == PackageManager::from(CargoBinstall)));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(CargoBinstall), "cargo binstall test" => "test");
        catch!(PackageManager::from(CargoBinstall), "cargo binstall test@1.2.3" => "test@1.2.3");

        // Multiple
        catch!(PackageManager::from(CargoBinstall), "cargo binstall test test2" => "test", "test2");
        catch!(PackageManager::from(CargoBinstall), "cargo binstall test install test-install" => "test", "install", "test-install");

        // Shouldn't match
        catch!(PackageManager::from(CargoBinstall), "cargo uninstall test test2" => ());
        catch!(PackageManager::from(CargoBinstall), "cargo binstall ." => ());
        catch!(PackageManager::from(CargoBinstall), "cargo install test" => ());

        // Flags that should be captured
        catch!(PackageManager::from(CargoBinstall), "cargo binstall --git https://test.com/test.git" => "https://test.com/test.git" ["--git"]);
    }
}
