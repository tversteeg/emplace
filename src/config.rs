use anyhow::{anyhow, Result};
use log::info;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    string::String,
};

/// Repository specific configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    pub url: String,
    #[serde(default = "RepoConfig::default_branch")]
    pub branch: String,
    #[serde(default = "RepoConfig::default_file")]
    pub file: String,
}

impl RepoConfig {
    fn new(url: String) -> Self {
        Self {
            url,
            branch: RepoConfig::default_branch(),
            file: RepoConfig::default_file(),
        }
    }

    fn default_branch() -> String {
        "master".to_owned()
    }

    fn default_file() -> String {
        ".emplace".to_owned()
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.file.clone())
    }
}

/// Emplace configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_mirror_dir_string")]
    pub repo_directory: String,
    pub repo: RepoConfig,
}

impl Config {
    /// Create a new config and save it on disk.
    pub fn new() -> Result<Self> {
        info!("No configuration file found.");
        let repo_url = dialoguer::Input::<String>::new()
            .with_prompt("The URL of the git repository you (want to) store the mirrors in")
            .interact()?;

        let config = Config {
            repo_directory: Config::default_mirror_dir_string(),
            repo: RepoConfig::new(repo_url),
        };

        // Save the config
        config.save_to_default_path()?;

        Ok(config)
    }

    /// Try to open the default config or create a new one.
    pub fn from_default_file_or_new() -> Result<Self> {
        match Config::from_default_file()? {
            Some(config) => Ok(config),
            None => Config::new(),
        }
    }

    /// Try to open the default.
    pub fn from_default_file() -> Result<Option<Self>> {
        Config::from_path(&Config::default_path())
    }

    /// Load the config from a file.
    pub fn from_path<P: AsRef<Path>>(file_path: &P) -> Result<Option<Self>> {
        if !file_path.as_ref().exists() {
            return Ok(None);
        }

        // Open the file
        let mut file = File::open(file_path)?;

        // Read the contents
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Some(toml::from_str(&contents)?))
    }

    /// Persist the config on disk.
    pub fn save_to_default_path(&self) -> Result<()> {
        fs::write(
            Config::default_path(),
            // Hardcode the default TOML config
            toml::to_string(self)?,
        )?;

        info!(
            "Config saved to: \"{}\".",
            Config::default_path().to_str().unwrap()
        );
        info!("You can edit the git repository URL and other settings here later.");

        Ok(())
    }

    pub fn full_file_path(&self) -> PathBuf {
        let mut base = PathBuf::from(self.repo_directory.clone());
        base.push(self.repo.path());

        base
    }

    fn default_path() -> PathBuf {
        dirs::config_dir()
            .expect("Could not find config dir")
            .join("emplace.toml")
    }

    fn default_mirror_dir() -> PathBuf {
        dirs::data_local_dir()
            .expect("Could not find local data dir")
            .join("emplace")
    }

    fn default_mirror_dir_string() -> String {
        Config::default_mirror_dir()
            .to_str()
            .expect("Could not get directory")
            .to_string()
    }
}
