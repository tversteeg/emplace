use git2::Repository;
use std::{
    path::Path,
    error::Error,
};

use crate::config::Config;

pub struct Repo {
    config: Config,
    repo: Repository,
}

impl Repo {
    pub fn new(config: Config) -> Result<Option<Self>, Box<dyn Error>> {
        let mirror_directory = config.mirror_directory.clone().expect("Mirror directory not set");
        let repo_url = config.repo.url.clone().expect("Repository URL not set");
        let repo_branch = config.repo.branch.clone().expect("Repository branch not set");

        let path = Path::new(&mirror_directory);
        let path_str = path.to_str().expect("Could not get directory").to_string();

        let is_new = !path.join(".git").exists();
        let repo = match is_new {
            false => {
                println!("Opening existing repo: \"{}\"", path_str);
                Repository::open(path)?
            },
            true => {
                println!("Cloning repo \"{}\" to \"{}\"", repo_url, path_str);
                Repository::clone(&*repo_url, path)?
            }
        };

        // Always pull
        pull(&repo, &*repo_branch)?;

        Ok(Some(Repo {
            config,
            repo
        }))
    }
}

fn pull(repo: &Repository, branch: &str) -> Result<(), git2::Error> {
    println!("Pulling origin/{}..", branch);

    // Do a fetch
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[branch], None, None)?;

    // Get the FETCH_HEAD commit
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    // Do a merge
    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", branch);
        let mut reference = repo.find_reference(&refname)?;

        fast_forward(repo, &mut reference, &fetch_commit)?;
    } else if analysis.0.is_normal() {
        panic!("Unhandled normal merge situation");
    }

    Ok(())
}

fn fast_forward(repo: &Repository, local_ref: &mut git2::Reference, remote_commit: &git2::AnnotatedCommit) -> Result<(), git2::Error> {
    let name = match local_ref.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(local_ref.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, remote_commit.id());
    local_ref.set_target(remote_commit.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(None)?;

    Ok(())
}
