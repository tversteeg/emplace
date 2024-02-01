use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Nix;

impl PackageManagerTrait for Nix {
    fn full_name(self) -> &'static str {
        "Nix"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["nix-env"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["-i", "--install", "-iA"]
    }

    fn install_command(self) -> &'static str {
        "nix-env -iA -g"
    }

    fn needs_root(self) -> bool {
        false
    }

    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("nix-env -q | grep -q {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        // TODO
        vec![]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![CaptureFlag::DynamicValue("-f")]
    }

    fn invalidating_flags(self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Nix;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::single_from_line("nix-env -iA nixos.test").unwrap();
        assert_eq!(manager, PackageManager::from(Nix));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Nix), "nix-env -i nixos.test" => "nixos.test");
        catch!(PackageManager::from(Nix), "nix-env -iA nixos.test" => "nixos.test");
        catch!(PackageManager::from(Nix), "nix-env --install nixos.test" => "nixos.test");

        catch!(PackageManager::from(Nix), "nix-env -iA nixpkgs.test" => "nixpkgs.test");

        catch!(PackageManager::from(Nix), "sudo nix-env -f 'stable-2.0' -iA nixos.test" => "nixos.test" ["-f 'stable-2.0'"]);

        // Ignore
        catch!(PackageManager::from(Nix), "sudo nix-env -q test" => ());
    }
}
