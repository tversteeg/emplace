use super::{PackageInstalledMethod, PackageManagerTrait};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
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

    fn capture_flags(self) -> Vec<(&'static str, Option<&'static str>)> {
        vec![("-t", Some("experimental"))]
    }
}

#[cfg(test)]
mod tests {
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::from_line("apt install test").unwrap();
        assert_eq!(manager, PackageManager::Apt);
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::Apt, "sudo apt install test" => "test");
        catch!(PackageManager::Apt, "apt install test" => "test");
        catch!(PackageManager::Apt, "sudo apt-get install test" => "test");
        catch!(PackageManager::Apt, "sudo apt install lib32gfortran5-x32-cross" => "lib32gfortran5-x32-cross");
        catch!(PackageManager::Apt, "sudo apt install linux-perf-5.3" => "linux-perf-5.3");

        // Multiple
        catch!(PackageManager::Apt, "sudo apt install test test2" => "test", "test2");

        // Ignore
        catch!(PackageManager::Apt, "sudo apt test test2" => ());

        // Flags with another regular argument
        catch!(PackageManager::Apt, "sudo apt -qq install test" => "test");
        catch!(PackageManager::Apt, "sudo apt install -f" => ());
        catch!(PackageManager::Apt, "sudo apt install test -f" => "test");
        catch!(PackageManager::Apt, "sudo apt install -c file test" => "test");

        // Flags that should be captured
        catch!(PackageManager::Apt, "sudo apt install -t experimental test" => "test" ["-t experimental"]);
    }
}
