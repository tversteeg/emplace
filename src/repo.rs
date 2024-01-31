use crate::{config::Config, git, migrate::zero_two, package::Packages};
use anyhow::{Context, Result};
use log::debug;
use ron::{
    de,
    ser::{to_string_pretty, PrettyConfig},
};
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(not(unix))]
use std::os::windows::fs::symlink_file as symlink;
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

/// Git repository where the emplace file lives.
#[derive(Debug)]
pub struct Repo {
    config: Config,
    path: PathBuf,
}

impl Repo {
    pub fn new(config: Config, pull_if_exists: bool) -> Result<Self> {
        debug!("Retrieving repository");

        let repo_directory = config.repo_directory.clone();
        let repo_url = config.repo.url.clone();
        let repo_branch = config.repo.branch.clone();

        let path = Path::new(&repo_directory);
        let path_str = path.to_str().expect("Could not get directory").to_string();

        let repo_exists = path.join(".git").exists();
        if repo_exists {
            println!("Opening Emplace repo: \"{}\".", path_str);

            if pull_if_exists {
                git::pull(&path, &repo_branch).context("pulling existing repo from config")?;
            }
        } else {
            println!("Cloning Emplace repo \"{}\" to \"{}\".", repo_url, path_str);

            fs::create_dir_all(path).context("creating new directory for repo")?;
            git::clone_single_branch(&path, &repo_url, &repo_branch).context("cloning new repo")?;

            // Create the emplace file if it doesn't exist
            let emplace_file = config.full_file_path();
            if !emplace_file.exists() {
                // If the repo contains a configuration file create a symbolic link to that,
                // otherwise create a new configuration file
                let repo_config_file = path.join("emplace.toml");
                if repo_config_file.exists() {
                    // Create a symbolic link
                    symlink(repo_config_file, &emplace_file)?;
                } else {
                    // Create a new configuration file
                    let empty_packages = Packages::empty();
                    let toml_string = to_string_pretty(&empty_packages, Repo::pretty_config())?;
                    fs::write(&emplace_file, toml_string)?;
                }
            }
        }

        Ok(Repo {
            config,
            path: path.to_path_buf(),
        })
    }

    /// Perform a git pull on the repository.
    pub fn pull(&self) -> Result<()> {
        let repo_branch = self.config.repo.branch.clone();
        git::pull(&self.path, &repo_branch).context("pulling repository")?;

        Ok(())
    }

    pub fn read(&self) -> Result<Packages> {
        // Open the file
        let mut file = File::open(self.config.full_file_path())
            .context("failed opening Emplace mirrors file")?;

        // Read the contents
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("reading packages string from repository")?;

        // Deserialize the file into the struct
        match de::from_str(&contents) {
            Ok(packages) => Ok(packages),
            // Deserializing failed, try to migrate from a previous version
            Err(err) => {
                // Try to migrate from emplace version <= 0.2
                if let Some(packages) = zero_two::try_migrate(&contents) {
                    return Ok(packages);
                }

                // Return the original error if migration failed
                Err(err).context("deserializing packages from repository")
            }
        }
    }

    pub fn mirror(&self, mut commands: Packages) -> Result<()> {
        // Get the message first before the old stuff is added
        let mut commit_msg = commands.commit_message();

        let full_path = self.config.full_file_path();
        if full_path.exists() {
            // A file already exists, merge the existing one with the current one
            let mut old: Packages = self.read()?;

            // Merge it with the new one
            commands.merge(&mut old);
        }

        // There's no file yet, just serialize everything and write it to a new file
        let toml_string = to_string_pretty(&commands, Repo::pretty_config())?;

        fs::write(&full_path, toml_string)?;

        // Add the file to git
        git::add_file(&self.path, &self.config.repo.file)?;

        // Check if there are other changes
        if git::has_changes(&self.path)? {
            commit_msg.push_str("\nIncluding changes of other files in the repository.");
            git::add_all_files(&self.path)?;
        }

        println!("Committing with message \"{}\".", commit_msg);
        git::commit_all(&self.path, &commit_msg, false)?;

        println!("Pushing to remote.");
        git::push(&self.path)?;

        Ok(())
    }

    pub fn clean(&self, commands: Packages) -> Result<()> {
        // Overwrite the file
        let toml_string = to_string_pretty(&commands, Repo::pretty_config())?;

        let full_path = self.config.full_file_path();
        fs::write(full_path, toml_string)?;

        let commit_msg = "Emplace - clean packages";
        println!("Committing with message \"{}\".", commit_msg);
        git::add_file(&self.path, &self.config.repo.file)?;
        git::commit_all(&self.path, commit_msg, false)?;

        println!("Pushing to remote.");
        git::push(&self.path)?;

        Ok(())
    }

    fn pretty_config() -> PrettyConfig {
        PrettyConfig::new().depth_limit(2).indentor("".into())
    }
}
