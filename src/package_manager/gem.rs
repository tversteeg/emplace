use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Gem;

impl PackageManagerTrait for Gem {
    fn full_name(self) -> &'static str {
        "Ruby Gem"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["gem"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "gem install"
    }

    fn needs_root(self) -> bool {
        false
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("gem -i {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec![
            "-n",
            "--bindir",
            "--document",
            "--build-root",
            "-P",
            "--trust-policy",
            "-g",
            "--file",
            "--without",
            "-s",
            "--source",
            "-B",
            "--bulk-treshold",
            "-p",
            "--http-proxy",
            "--config-file",
        ]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![
            CaptureFlag::DynamicValue("-i"),
            CaptureFlag::DynamicValue("--install-dir"),
            CaptureFlag::DynamicValue("--platform"),
            CaptureFlag::DynamicValue("-v"),
            CaptureFlag::DynamicValue("--version"),
        ]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Gem;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("gem install test").unwrap();
        assert_eq!(manager, PackageManager::from(Gem));
    }

    #[test]
    fn test_catch() {
        catch!(PackageManager::from(Gem), "gem install test" => "test");
    }

    #[test]
    fn test_catch_flags() {
        catch!(PackageManager::from(Gem), "gem install -i ~/some_dir test" => "test" ["-i ~/some_dir"]);
    }
}
