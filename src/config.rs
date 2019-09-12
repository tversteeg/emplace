use serde::{Serialize, Deserialize};
use std::{
    path::{Path, PathBuf},
    fs::{self, File},
    error::Error,
    string::String,
    io::Read,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    pub url: String,
    pub username: String,
    #[serde(default = "RepoConfig::default_branch")]
    pub branch: String,
    #[serde(default = "RepoConfig::default_file")]
    pub file: String,
}

impl RepoConfig {
    fn new(url: String, username: String) -> Self {
        Self {
            url,
            username,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_mirror_dir_string")]
    pub mirror_directory: String,
    pub repo: RepoConfig
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        println!("No configuration file found.");
        let repo_url = dialoguer::Input::<String>::new()
            .with_prompt("The URL of the git repository you (want to) store the mirrors in")
            .interact()?;

        let username = dialoguer::Input::<String>::new()
            .with_prompt("The username of the account you want to push with")
            .interact()?;

        let config = Config {
            mirror_directory: Config::default_mirror_dir_string(),
            repo: RepoConfig::new(repo_url, username)
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

    pub fn full_file_path(&self) -> PathBuf {
        let mut base = PathBuf::from(self.mirror_directory.clone());
        base.push(self.repo.path());

        base
    }

    fn default_path() -> PathBuf {
        dirs::config_dir().expect("Could not find config dir")
            .join("emplace.toml")
    }

    fn default_mirror_dir() -> PathBuf {
        dirs::data_local_dir().expect("Could not find local data dir")
            .join("emplace")
    }

    fn default_mirror_dir_string() -> String {
         Config::default_mirror_dir()
            .to_str().expect("Could not get directory")
            .to_string()
    }
}
