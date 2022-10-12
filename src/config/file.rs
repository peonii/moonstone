use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub repo_link: String,
    pub repo_branch: String,
    pub gen_compile_command: String,
    pub brute_compile_command: String,
    pub main_compile_command: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            repo_link: "https://github.com/peonii/mst-defaults.git".to_string(),
            repo_branch: "main".to_string(),
            gen_compile_command: "g++ gen.cpp -o gen".to_string(),
            brute_compile_command: "g++ brute.cpp -o brute".to_string(),
            main_compile_command: "g++ main.cpp -o main".to_string(),
        }
    }

    /**
     * Loads the config from the config file
     * The config file is located at `~/.mst/config.toml`
     */
    pub fn load() -> Result<Self, Error> {
        let home_directory = match home::home_dir() {
            Some(path) => path,
            None => return Err("Could not find home directory".into()),
        };

        let config_path = home_directory.join(".mst").join("config.toml");

        if !config_path.try_exists()? {
            return Err("Config file does not exist".into());
        }

        let config_file = std::fs::read_to_string(config_path)?;
        let config: Self = toml::from_str(&config_file)?;

        Ok(config)
    }

    /**
     * Saves the current instance of the config to the default path
     * The default path is `~/.mst/config.toml`
     */
    pub fn save(&self) -> Result<(), Error> {
        let home_directory = match home::home_dir() {
            Some(path) => path,
            None => return Err("Could not find home directory".into()),
        };

        let config_path = home_directory.join(".mst");

        if !config_path.try_exists()? {
            std::fs::create_dir_all(&config_path)?;
        }

        let config_file = toml::to_string(self)?;

        std::fs::write(config_path.join("config.toml"), config_file)?;

        Ok(())
    }
}
