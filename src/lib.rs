#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
// THIS IS TEMPORARY, REMOVE THIS WHEN DOING TESTING
#![allow(clippy::unused_async)]

mod args;
mod config;
mod project;
mod cache;

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
        repo.clone_repo(&config.repo_link)?;
        println!("It looks like this is your first time using Moonstone. A config file has been created at {}.", "~/.mst/config.toml".bright_blue());
    }

    let command_result = args::match_command().await;

    if let Err(e) = command_result {
        eprintln!("{} {e}", "Error!".red().bold());
        return Err(e);
    }

    Ok(())
}