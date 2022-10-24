use std::fs;

use crate::config::file::Config;
use crate::Error;

use super::repo::RepoCache;

pub fn reset_cache() -> Result<(), Error> {
    if !casual::confirm(
        "Are you sure you want to reset your cache? This will remove ALL of your cached templates.",
    ) {
        return Err("User said no to resetting cache".into());
    }

    println!("⏳ Resetting cache...");
    let cache = RepoCache::new();
    cache.save()?;

    let home_directory = match home::home_dir() {
        Some(path) => path,
        None => return Err("Could not find home directory".into()),
    };

    let cache_path = home_directory.join(".mst").join("cache");

    fs::remove_dir_all(&cache_path)?;
    fs::create_dir(&cache_path)?;

    println!("✅ Cache successfully reset.");

    Ok(())
}

pub fn reset_config() -> Result<(), Error> {
    if !casual::confirm(
        "Are you sure you want to reset your config? This will wipe all of your configuration.",
    ) {
        return Err("User said no to resetting config".into());
    }

    println!("⏳ Resetting config...");

    let home_directory = match home::home_dir() {
        Some(path) => path,
        None => return Err("Could not find home directory".into()),
    };

    let config_path = home_directory.join(".mst").join("config.toml");

    if config_path.exists() {
        fs::remove_file(config_path)?;
    }
    
    let config = Config::new();
    config.save()?;

    println!("✅ Config successfully reset.");
    Ok(())
}
