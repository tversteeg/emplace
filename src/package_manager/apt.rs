use super::{PackageInstalledMethod, PackageManagerTrait};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Apt;

impl PackageManagerTrait for Apt {
    fn full_name(self) -> &'static str {
        "Advance Package Tool"
    }

    fn command(self) -> &'static str {
        "apt"
    }

    fn sub_command(self) -> &'static str {
        "install"
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

    fn capture_flags(self) -> Vec<&'static str> {
        vec!["-t experimental"]
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
        catch!(PackageManager::Apt, "sudo apt install test" => "test");
        catch!(PackageManager::Apt, "sudo apt install test test2" => "test", "test2");
        catch!(PackageManager::Apt, "sudo apt test test2" => ());
    }
}
