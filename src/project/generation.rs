use crate::cache::repo::RepoCache;
use crate::project::file::Project;
use crate::{cwd, Error};
use colored::Colorize;

pub fn new_project(name: Option<&String>) -> Result<(), Error> {
    println!("{} 🔍 Looking up cache...", "[1/2]".dimmed());
    let mut project_path = cwd!()?;

    if let Some(name) = name {
        project_path = project_path.join(name);
    }

    let project = Project::new(project_path.as_path());
    project.save()?;

    let mut cache = RepoCache::load()?;
    let current_repo = crate::config::file::Config::load()?;

    let repo_name = current_repo.repo_link;
    let repo_branch = current_repo.repo_branch;

    if !cache.exists(&repo_name, &repo_branch) {
        cache.clone_repo(&repo_name, &repo_branch)?;
    }

    println!("{} 📁 Creating boilerplate files...", "[2/2]".dimmed());

    let repo_path = RepoCache::get_path_of_repo(&repo_name, &repo_branch)?;

    // copy main.cpp, gen.cpp and brute.cpp into the project directory
    std::fs::copy(repo_path.join("main.cpp"), project_path.join("main.cpp"))?;
    std::fs::copy(repo_path.join("brute.cpp"), project_path.join("brute.cpp"))?;
    std::fs::copy(repo_path.join("gen.cpp"), project_path.join("gen.cpp"))?;

    std::fs::create_dir(project_path.join("tests"))?;

    println!(
        "✅ {} new project in {}!",
        "Created".green(),
        project_path.display()
    );

    Ok(())
}
