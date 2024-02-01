use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Guix;

impl PackageManagerTrait for Guix {
    fn full_name(self) -> &'static str {
        "GNU Guix"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["guix"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "guix install"
    }

    fn needs_root(self) -> bool {
        false
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("guix --list-installed=\"^{}$\"", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec![
            "-L",
            "--load-path",
            "-v",
            "--verbosity",
            "--max-silent-time",
            "-c",
            "--cores",
            "-M",
            "--max-jobs",
            "--debug",
        ]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![
            CaptureFlag::DynamicValue("-p"),
            CaptureFlag::DynamicValue("--profile"),
        ]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Guix;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("guix install emplace").unwrap();
        assert_eq!(manager, PackageManager::from(Guix));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Guix), "guix install emplace" => "emplace");
        catch!(PackageManager::from(Guix), "guix install -c 8 emplace" => "emplace");

        // Ignore
        catch!(PackageManager::from(Guix), "guix -q test" => ());
    }
}
