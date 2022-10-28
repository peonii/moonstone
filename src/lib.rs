#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]

mod args;
mod cache;
mod config;
mod project;
mod replace;
mod testing;
mod utils;

use colored::Colorize;
use config::file::Config;

use crate::cache::repo::RepoCache;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/**
The main function of the library.

# Errors
* Every error that is returned is already printed to the console.
* No need to handle them yourself.
*/
pub async fn lib_main() -> Result<(), Error> {
    // Check for first load and create config if needed
    if Config::load().is_err() {
        let config = Config::new();
        config.save()?;
        let mut repo = RepoCache::new();
        repo.clone_repo(&config.repo_link, &config.repo_branch)?;

        if let Some(home_path) = home::home_dir() {
            std::fs::create_dir(home_path.join(".mst").join("libs"))?;
        }

        println!("It looks like this is your first time using Moonstone. A config file has been created at {}.", "~/.mst/config.toml".bright_blue());
    }

    let command_result = args::match_command().await;

    if let Err(e) = command_result {
        eprintln!("{} {e}", "Error!".red().bold());
        return Err(e);
    }

    Ok(())
}
