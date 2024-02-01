use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Go;

impl PackageManagerTrait for Go {
    fn full_name(self) -> &'static str {
        "Go"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["go"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["get"]
    }

    fn install_command(self) -> &'static str {
        "go get -u"
    }

    fn needs_root(self) -> bool {
        false
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("go list ... | grep -q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
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
    use super::Go;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("go get test").unwrap();
        assert_eq!(manager, PackageManager::from(Go));
    }

    #[test]
    fn test_catch() {
        catch!(PackageManager::from(Go), "go get test" => "test");
    }
}
