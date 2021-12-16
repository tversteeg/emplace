/// Automatic migration for version <= 0.2.
pub mod zero_two {
    use crate::{
        package::{Package, Packages},
        package_manager::*,
    };
    use ron::de;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    enum OldPackageManager {
        Apt,
        Cargo,
        Pacman,
        Yay,
        Rua,
        Snap,
        Chocolatey,
        Scoop,
        Pip,
        Pip3,
        Npm,
        Nix,
        // Mapped to Rustup
        RustupComponent,
        // Mapped to Pip
        PipUser,
        // Mapped to Pip3
        Pip3User,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct OldPackage {
        source: OldPackageManager,
        name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct OldPackages(Vec<OldPackage>);

    /// Attempt to migrate a version 0.2 package list to the current version.
    pub fn try_migrate(contents: &str) -> Option<Packages> {
        // Parse the ron and return if it fails
        let old_packages: OldPackages = de::from_str(contents).ok()?;

        println!("Migrating from Emplace 0.2");

        Some(
            old_packages
                .0
                .into_iter()
                .map(|old_package| {
                    // Map PipUser to Pip
                    if old_package.source == OldPackageManager::PipUser {
                        return Package::new(
                            PackageManager::from(Pip),
                            old_package.name,
                            vec!["--user".into()],
                        );
                    }
                    // Map Pip3User to Pip3
                    if old_package.source == OldPackageManager::Pip3User {
                        return Package::new(
                            PackageManager::from(Pip3),
                            old_package.name,
                            vec!["--user".into()],
                        );
                    }

                    let source = match old_package.source {
                        OldPackageManager::Apt => PackageManager::from(Apt),
                        OldPackageManager::Cargo => PackageManager::from(Cargo),
                        OldPackageManager::Pacman => PackageManager::from(Pacman),
                        OldPackageManager::Yay => PackageManager::from(Yay),
                        OldPackageManager::Rua => PackageManager::from(Rua),
                        OldPackageManager::Snap => PackageManager::from(Snap),
                        OldPackageManager::Chocolatey => PackageManager::from(Chocolatey),
                        OldPackageManager::Scoop => PackageManager::from(Scoop),
                        OldPackageManager::Pip => PackageManager::from(Pip),
                        OldPackageManager::Pip3 => PackageManager::from(Pip3),
                        OldPackageManager::Npm => PackageManager::from(Npm),
                        OldPackageManager::Nix => PackageManager::from(Nix),
                        // Rename RustupComponent to Rustup
                        OldPackageManager::RustupComponent => PackageManager::from(Rustup),
                        _ => unreachable!(),
                    };

                    if old_package.name.starts_with("--git") {
                        // Convert --git names to flags
                        return Package::new(
                            source,
                            old_package.name.split_ascii_whitespace().skip(1).collect(),
                            vec!["--git".into()],
                        );
                    }

                    Package::new(source, old_package.name, vec![])
                })
                .collect::<Vec<Package>>()
                .into(),
        )
    }
}
