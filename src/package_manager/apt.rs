use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Apt;

impl PackageManagerTrait for Apt {
    fn full_name(self) -> &'static str {
        "Advance Package Tool"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["apt", "apt-get"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "apt-get install -y"
    }

    fn needs_root(self) -> bool {
        true
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("dpkg -s {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec!["-c", "--config-file", "-o", "--option", "-q", "--quiet"]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![CaptureFlag::SetValue("-t", "experimental")]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Apt;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("apt install test").unwrap();
        assert_eq!(manager, PackageManager::from(Apt));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Apt), "sudo apt install test" => "test");
        catch!(PackageManager::from(Apt), "apt install test" => "test");
        catch!(PackageManager::from(Apt), "sudo apt-get install test" => "test");
        catch!(PackageManager::from(Apt), "sudo apt install lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::from(Apt), "sudo apt install linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::from(Apt), "sudo apt install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::from(Apt), "sudo apt test test2" => ());
        catch!(PackageManager::from(Apt), "sudo apt install -f" => ());
        catch!(PackageManager::from(Apt), "sudo bapt install test" => ());
    }

    #[test]
    fn test_catch_flags() {
        // Flags with another regular argument
        catch!(PackageManager::from(Apt), "sudo apt -qq install test" => "test");
        catch!(PackageManager::from(Apt), "sudo apt install test -f" => "test");
        catch!(PackageManager::from(Apt), "sudo apt install -c file test" => "test");

        // Flags that should be captured
        catch!(PackageManager::from(Apt), "sudo apt install -t experimental test" => "test" ["-t experimental"]);
    }
}
