use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Brew;

impl PackageManagerTrait for Brew {
    fn full_name(self) -> &'static str {
        "Homebrew"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["brew"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "brew install"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("brew list {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec!["--env", "--cc"]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![
            CaptureFlag::Single("cask"),
            CaptureFlag::Single("--devel"),
            CaptureFlag::Single("--HEAD"),
            CaptureFlag::Single("--fetch-HEAD"),
        ]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Brew;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("brew install test").unwrap();
        assert_eq!(manager, PackageManager::from(Brew));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Brew), "sudo brew install test" => "test");
        catch!(PackageManager::from(Brew), "brew install test" => "test");
        catch!(PackageManager::from(Brew), "sudo brew install lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Brew), "sudo brew install linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Brew), "sudo brew install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Brew), "sudo brew test test2" => ());
    }
}
