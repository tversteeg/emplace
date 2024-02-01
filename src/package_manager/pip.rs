use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pip;

impl PackageManagerTrait for Pip {
    fn full_name(self) -> &'static str {
        "Python Pip"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["pip"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "pip install -q"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("pip show -q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        // TODO
        vec![]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![CaptureFlag::Single("--user")]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec!["-r"]
    }
}

#[cfg(test)]
mod tests {
    use super::Pip;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("pip install test").unwrap();
        assert_eq!(manager, PackageManager::from(Pip));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Pip), "sudo pip install test" => "test");
        catch!(PackageManager::from(Pip), "pip install test" => "test");

        // Flags
        catch!(PackageManager::from(Pip), "pip install --user test" => "test" ["--user"]);

        // Ignore
        catch!(PackageManager::from(Pip), "sudo pip3 install test test2" => ());
        catch!(PackageManager::from(Pip), "sudo pip test test2" => ());
        catch!(PackageManager::from(Pip), "pip install -r requirements.txt" => ());
    }
}
