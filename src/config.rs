use serde::Deserialize;
use std::{
    path::Path,
    fs::File,
    error::Error,
    string::String,
    io::Read,
    default::Default,
};

#[derive(Debug, Deserialize)]
pub struct Config {
    git_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            git_dir: None
        }
    }
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(file_path: &P) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let mut file = File::open(file_path)?;

        // Read the contents
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize the file into the struct
        let cfg: Config = toml::from_str(&*contents)?;

        Ok(cfg)
    }
}
