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
    #[serde(default = "RepoConfig::default_branch")]
    pub branch: String,
    #[serde(default = "RepoConfig::default_file")]
    pub file: String,
    #[serde(default = "RepoConfig::default_private_key")]
    pub ssh_private_key: String,
    #[serde(default = "RepoConfig::default_public_key")]
    pub ssh_public_key: String,
}

impl RepoConfig {
    fn new(url: String) -> Self {
        Self {
            url,
            ssh_private_key: RepoConfig::default_private_key(),
            ssh_public_key: RepoConfig::default_public_key(),
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

    fn default_private_key() -> String {
         dirs::home_dir().expect("Could not get home directory")
             .join(".ss/id_rsa")
            .to_str().expect("Could not get directory")
            .to_string()
    }

    fn default_public_key() -> String {
         dirs::home_dir().expect("Could not get home directory")
             .join(".ssh/id_rsa.pub")
            .to_str().expect("Could not get directory")
            .to_string()
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.file.clone())
    }

    pub fn private_key_path(&self) -> PathBuf {
        PathBuf::from(self.ssh_private_key.clone())
    }

    pub fn public_key_path(&self) -> PathBuf {
        PathBuf::from(self.ssh_public_key.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_mirror_dir_string")]
    pub repo_directory: String,
    pub repo: RepoConfig
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        println!("No configuration file found.");
        let repo_url = dialoguer::Input::<String>::new()
            .with_prompt("The URL of the git repository you (want to) store the mirrors in")
            .interact()?;

        let config = Config {
            repo_directory: Config::default_mirror_dir_string(),
            repo: RepoConfig::new(repo_url)
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
        let mut base = PathBuf::from(self.repo_directory.clone());
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
