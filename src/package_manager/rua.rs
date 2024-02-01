use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rua;

impl PackageManagerTrait for Rua {
    fn full_name(self) -> &'static str {
        "RUA"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["rua"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "rua install"
    }

    fn needs_root(self) -> bool {
        false
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("rua search {}", package))
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
    use super::Rua;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("rua install test").unwrap();
        assert_eq!(manager, PackageManager::from(Rua));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Rua), "rua install test" => "test");
        catch!(PackageManager::from(Rua), "rua install lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Rua), "rua install linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Rua), "rua install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Rua), "rua test test2" => ());
    }
}
