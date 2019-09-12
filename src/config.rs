use serde::{Serialize, Deserialize};
use std::{
    path::{Path, PathBuf},
    fs::{self, File},
    error::Error,
    string::String,
    io::Read,
    default::Default,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    pub url: Option<String>,
    pub branch: Option<String>,
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            url: None,
            branch: Some("master".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mirror_directory: Option<String>,
    pub repo: RepoConfig
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mirror_directory: Some(Config::default_mirror_dir()
                .to_str().expect("Could not get directory")
                .to_string()),
            repo: RepoConfig::default()
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        println!("No configuration file found.");
        let repo_url = dialoguer::Input::<String>::new()
            .with_prompt("The URL of the git repository you (want to) store the mirrors in")
            .interact()?;

        let config = Config {
            repo: RepoConfig {
                url: Some(repo_url),
                ..Default::default()
            },
            ..Default::default()
        };

        // Save the config
        config.save_to_default_path()?;

        Ok(config)
    }

    pub fn from_default_file() -> Result<Option<Self>, Box<dyn Error>> {
        Config::from_path(&Config::default_path())
    }

    pub fn from_path<P: AsRef<Path>>(file_path: &P) -> Result<Option<Self>, Box<dyn Error>> {
        if !file_path.as_ref().exists() {
            return Ok(None)
        }

        // Open the file
        let mut file = File::open(file_path)?;

        // Read the contents
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize the file into the struct
        let cfg: Config = toml::from_str(&*contents)?;

        Ok(Some(cfg))
    }

    pub fn save_to_default_path(&self) -> Result<(), Box<dyn Error>> {
        let toml_string = toml::to_string(self)?;

        fs::write(Config::default_path(), toml_string)?;

        println!("Config saved to: \"{}\".", Config::default_path().to_str().expect("Could not unwrap path to string"));
        println!("You can edit the git repository URL and other settings here later.");

        Ok(())
    }

    fn default_path() -> PathBuf {
        dirs::config_dir().expect("Could not find config dir")
            .join("emplace.toml")
    }

    fn default_mirror_dir() -> PathBuf {
        dirs::data_local_dir().expect("Could not find local data dir")
            .join("emplace")
    }
}
