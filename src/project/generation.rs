use crate::Error;
use crate::cache::repo::RepoCache;
use crate::project::file::Project;

pub fn new_project(name: Option<&String>) -> Result<(), Error> {
    let mut project_path = std::env::current_dir()?;
    if let Some(name) = name {
        project_path = project_path.join(name);
    }

    let project = Project::new(project_path.as_path());
    project.save()?;

    let mut cache = RepoCache::load()?;
    let current_repo = crate::config::file::Config::load()?.repo_link;

    if !cache.exists(&current_repo) {
        cache.clone_repo(&current_repo)?;
    }

    let repo_path = RepoCache::get_path_of_repo(&current_repo)?;

    // copy main.cpp, gen.cpp and brute.cpp into the project directory
    std::fs::copy(repo_path.join("main.cpp"), project_path.join("main.cpp"))?;
    std::fs::copy(repo_path.join("brute.cpp"), project_path.join("brute.cpp"))?;
    std::fs::copy(repo_path.join("gen.cpp"), project_path.join("gen.cpp"))?;

    Ok(())
}