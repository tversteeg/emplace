use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    error::Error,
    io::Read,
};

use crate::config::Config;
use crate::catch::Packages;
use crate::git;

pub struct Repo {
    config: Config,
    path: PathBuf,
}

impl Repo {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let repo_directory = config.repo_directory.clone();
        let repo_url = config.repo.url.clone();
        let repo_branch = config.repo.branch.clone();

        let path = Path::new(&repo_directory);
        let path_str = path.to_str().expect("Could not get directory").to_string();

        let repo_exists = path.join(".git").exists();
        if repo_exists {
            println!("Opening existing repo: \"{}\"", path_str);

            git::pull(&path, false)?;
        } else {
            println!("Cloning repo \"{}\" to \"{}\"", repo_url, path_str);

            fs::create_dir_all(path)?;
            git::clone_single_branch(&path, &*repo_url, &*repo_branch, false)?;
        }

        Ok(Repo {
            config,
            path: path.to_path_buf()
        })
    }

    pub fn mirror(&self, mut commands: Packages) -> Result<(), Box<dyn Error>> {
        // Get the message first before the old stuff is added
        let commit_msg = commands.commit_message();

        let full_path = self.config.full_file_path();
        if full_path.exists() {
            // A file already exists, merge the existing one with the current one
            self.merge_file(&mut commands)?;
        }

        // There's no file yet, just serialize everything and write it to a new file
        let toml_string = serde_json::to_string(&commands)?;
        fs::write(&full_path, toml_string)?;

        println!("Commiting with message \"{}\"..", commit_msg);
        git::add_file(&self.path, &*self.config.repo.file, false)?;
        git::commit_all(&self.path, &*commit_msg, false, false)?;

        println!("Pushing to remote");
        git::push(&self.path, false)?;

        Ok(())
    }

    pub fn merge_file(&self, commands: &mut Packages) -> Result<(), Box<dyn Error>> {
        // Open the file
        let mut file = File::open(&self.config.full_file_path())?;

        // Read the contents
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize the file into the struct
        let mut old: Packages = serde_json::from_str(&*contents)?;
        // Merge it with the new one
        commands.merge(&mut old);

        Ok(())
    }
}
