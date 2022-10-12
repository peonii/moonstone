use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Error;
use git2::build::RepoBuilder;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct RepoCache {
    pub repos: Vec<Repo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Repo {
    pub link: String,
    pub branch: String,
    pub hash: String,
}

impl RepoCache {
    pub const fn new() -> Self {
        Self { repos: Vec::new() }
    }

    /**
    Default cache path location is `~/.mst/.cacheindex.toml`
    */
    pub fn load() -> Result<Self, Error> {
        let home_directory = match home::home_dir() {
            Some(path) => path,
            None => return Err("Could not find home directory".into()),
        };

        let cache_path = home_directory.join(".mst").join(".cacheindex.toml");

        if !cache_path.try_exists()? {
            return Err("Cache file does not exist".into());
        }

        let cache_file = std::fs::read_to_string(cache_path)?;
        let cache: Self = toml::from_str(&cache_file)?;

        Ok(cache)
    }

    /**
    Default cache path location is `~/.mst/.cacheindex.toml`
    */
    pub fn save(&self) -> Result<(), Error> {
        let home_directory = match home::home_dir() {
            Some(path) => path,
            None => return Err("Could not find home directory".into()),
        };

        let cache_path = home_directory.join(".mst");

        if !cache_path.try_exists()? {
            std::fs::create_dir_all(&cache_path)?;
        }

        let cache_file = toml::to_string(self)?;

        std::fs::write(cache_path.join(".cacheindex.toml"), cache_file)?;

        Ok(())
    }

    pub fn clone_repo(&mut self, repo: &String, branch: &String) -> Result<(), Error> {
        let home_directory = match home::home_dir() {
            Some(path) => path,
            None => return Err("Could not find home directory".into()),
        };

        let repo_path = home_directory.join(".mst").join("cache");
        if !repo_path.try_exists()? {
            std::fs::create_dir_all(&repo_path)?;
        }

        // This is ugly

        // Generate a hash of the repo link (sha256, encoded in hex)
        let mut repo_name_hash = Sha256::new();
        repo_name_hash.update(repo);
        repo_name_hash.update(branch);
        let repo_name_hash = repo_name_hash.finalize();

        let repo_name_hash = hex::encode(&repo_name_hash);

        let repo_path = repo_path.join(&repo_name_hash);

        if repo_path.try_exists()? {
            return Err("Repo already exists".into());
        }

        let mut repo_builder = RepoBuilder::new();
        repo_builder.branch(branch);

        repo_builder.clone(repo, repo_path.as_path())?;
        self.repos.push(Repo {
            link: repo.clone(),
            branch: branch.clone(),
            hash: repo_name_hash.clone(),
        });
        self.save()
    }

    pub fn exists(&self, repo: &str, branch: &String) -> bool {
        for repo_obj in self.repos.iter() {
            if repo_obj.link == repo && &repo_obj.branch == branch {
                return true;
            }
        }
        false
    }

    pub fn get_path_of_repo(repo: &String, branch: &String) -> Result<PathBuf, Error> {
        let home_directory = match home::home_dir() {
            Some(path) => path,
            None => return Err("Could not find home directory".into()),
        };

        let repo_path = home_directory.join(".mst").join("cache");

        // Generate a hash of the repo link (sha256, encoded in hex)
        let mut repo_name_hash = Sha256::new();
        repo_name_hash.update(repo);
        repo_name_hash.update(branch);
        let repo_name_hash = repo_name_hash.finalize();

        let repo_name_hash = hex::encode(repo_name_hash);

        let repo_path = repo_path.join(repo_name_hash);

        Ok(repo_path)
    }
}
