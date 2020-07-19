use crate::git;
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    string::String,
};

/// A symbolic link that will be created by the install command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symlink {
    /// The relative path in the repository.
    pub source: String,
    /// The absolute path on disk.
    pub destination: String,
}

impl Symlink {
    /// Expanded destination path.
    pub fn expanded_destination(&self) -> String {
        shellexpand::tilde(&self.destination).into_owned()
    }
}

/// Repository specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_mirror_dir_string")]
    pub repo_directory: String,
    pub repo: RepoConfig,
    #[serde(default = "Vec::new", rename = "symlink")]
    pub symlinks: Vec<Symlink>,
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
            symlinks: Vec::new(),
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

    /// Ask the user if they want to change the repo path, clone it or create locally, or abort.
    pub fn clone_repo_ask(&mut self) -> Result<bool> {
        let term = console::Term::stdout();
        let theme: Box<dyn dialoguer::theme::Theme> = if term.features().colors_supported() {
            Box::new(dialoguer::theme::ColorfulTheme::default())
        } else {
            Box::new(dialoguer::theme::SimpleTheme)
        };
        let prompt = String::from("Choose what to do with the repository");
        let choices = &[
            "Change repository path before cloning it",
            "Clone the repo or create the repository locally",
            "Do nothing for now",
        ];
        let chosen = dialoguer::MultiSelect::with_theme(&*theme)
            .with_prompt(prompt)
            .items(choices)
            .interact()?;
        if chosen.contains(&2) {
            return Ok(false);
        }
        if chosen.contains(&0) {
            let repo_path = dialoguer::Input::<String>::with_theme(&*theme)
                .with_prompt("Where do you want your repository to be located")
                .interact()?;
            self.repo_directory = repo_path;
            self.save_to_default_path()?;
        }
        if chosen.contains(&1) {
            // Prompt is to stop it from blending with previous terminal output
            let prompt = "What do you want to do?";
            let choices_in = &["Clone the repo", "Create it locally"];
            let chosen_in = dialoguer::Select::with_theme(&*theme)
                .with_prompt(prompt)
                .items(choices_in)
                .paged(true)
                .clear(true)
                .interact_on(&term)?;
            if chosen_in == 0 {
                return git::clone_full(&self.repo_directory, &self.repo.url);
            } else {
                fs::DirBuilder::new()
                    .recursive(true)
                    .create(&self.repo_directory)?;
                git::init_repo(&self.repo_directory)?;
                return git::set_remote(&self.repo_directory, &self.repo.url);
            }
        }
        Ok(true)
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
        let result = self.save(Config::default_path());

        info!("You can edit the git repository URL and other settings here later.");

        result
    }

    /// Persist the config to a certain file.
    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Workaround to Issue #71
        // As suggested in issue #142 on toml-rs github repository
        // First convert the Config instance to a toml Value,
        // then serialize it to toml
        let value = toml::Value::try_from(self)?;
        fs::write(
            path.as_ref(),
            // Hardcode the default TOML config
            toml::to_string(&value)?,
        )?;
        info!("Config saved to: \"{}\".", path.as_ref().to_str().unwrap());

        Ok(())
    }

    /// Add a symbolic link line.
    pub fn add_symlink<S, D>(&mut self, source: S, destination: D)
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
    {
        self.symlinks.push(Symlink {
            source: source.as_ref().to_str().expect("path is invalid").into(),
            destination: destination
                .as_ref()
                .to_str()
                .expect("path is invalid")
                .into(),
        });
    }

    /// The repository path.
    pub fn folder_path(&self) -> PathBuf {
        PathBuf::from(&self.repo_directory)
    }

    /// The path of the .emplace file.
    pub fn full_file_path(&self) -> PathBuf {
        let mut base = PathBuf::from(&self.repo_directory);
        base.push(self.repo.path());

        base
    }

    /// Get the default path where the 'emplace.toml' file lives.
    pub fn default_path() -> PathBuf {
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
