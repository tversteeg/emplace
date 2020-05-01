use super::{PackageInstalledMethod, PackageManagerTrait};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Apt;

impl PackageManagerTrait for Apt {
    fn full_name() -> &'static str {
        "Advance Package Tool"
    }

    fn command() -> &'static str {
        "apt"
    }

    fn install_command() -> &'static str {
        "apt-get install -y"
    }

    fn needs_root() -> bool {
        true
    }

    fn is_installed(package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("dpkg -s {}", package))
    }
}
