use anyhow::{anyhow, Result};
use log::info;
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    string::String,
};
use toml_edit::{Document, Item, Table, Value};

/// Repository specific configuration.
#[derive(Debug)]
pub struct RepoConfig {
    pub url: String,
    pub branch: String,
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

    /// Construct the config from a TOML table, use default values if they are missing.
    fn from_toml(table: &Table) -> Result<Option<Self>> {
        // Get the section
        let section = match table.get("repo") {
            Some(section) => section,
            None => return Ok(None),
        }
        .as_table()
        .ok_or_else(|| anyhow!("repo is not a section"))?;

        let url = section
            .get("url")
            .ok_or_else(|| anyhow!("repo.url value is missing"))?
            .as_str()
            .ok_or_else(|| anyhow!("repo.url value is not a string"))?
            .to_string();

        let branch = section
            .get("branch")
            // If we couldn't get the value create a new TOML value
            .unwrap_or(&Item::Value(Value::from(Self::default_branch())))
            .as_str()
            .ok_or_else(|| anyhow!("repo.branch value is not a string"))?
            .to_string();

        let file = section
            .get("file")
            // If we couldn't get the value create a new TOML value
            .unwrap_or(&Item::Value(Value::from(Self::default_branch())))
            .as_str()
            .ok_or_else(|| anyhow!("repo.file value is not a string"))?
            .to_string();

        Ok(Some(Self { url, branch, file }))
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
#[derive(Debug)]
pub struct Config {
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

        // Deserialize the file into the struct
        let document = contents.parse::<Document>()?;
        let table = document.as_table();

        // Get the repo directory or use the default one
        let repo_directory = table
            .get("repo_directory")
            // If we couldn't get the value create a new TOML value
            .unwrap_or(&Item::Value(Value::from(Self::default_mirror_dir_string())))
            .as_str()
            .ok_or_else(|| anyhow!("repo_directory value is not a string"))?
            .to_string();

        let repo = match RepoConfig::from_toml(&table)? {
            Some(repo) => repo,
            // Section with URL is missing, we need this for a config
            None => return Ok(None),
        };

        Ok(Some(Config {
            repo_directory,
            repo,
        }))
    }

    /// Persist the config on disk.
    pub fn save_to_default_path(&self) -> Result<()> {
        fs::write(
            Config::default_path(),
            // Hardcode the default TOML config
            config_toml(
                &self.repo_directory,
                &self.repo.url,
                &self.repo.branch,
                &self.repo.file,
            ),
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

fn config_toml(repo_directory: &str, url: &str, branch: &str, file: &str) -> String {
    format!(
        r#"
repo_directory = "{repo_directory}"

[repo]
url = "{url}"
branch = "{branch}"
file = "{file}"
    "#,
        repo_directory = repo_directory,
        url = url,
        branch = branch,
        file = file
    )
}
