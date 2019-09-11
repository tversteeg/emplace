use git2::Repository;
use std::{
    path::Path,
    error::Error,
};

use crate::config::Config;

pub struct Repo {
    repo: Repository
}

impl Repo {
    pub fn new(config: Config) -> Result<Option<Self>, Box<dyn Error>> {
        let mirror_directory = config.mirror_directory.expect("Mirror directory not set");
        let repo_url = config.repo_url.expect("Repository URL not set");

        let path = Path::new(&mirror_directory);
        let repo = match path.exists() {
            true => Repository::open(path),
            false => Repository::clone(&*repo_url, path),
        }?;

        let repo = Repo {
            repo
        };

        Ok(Some(repo))
    }
}
