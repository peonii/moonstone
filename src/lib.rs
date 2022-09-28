use crate::config::Config;
use std::env;

pub mod config;
pub mod dotfiles;
pub mod project;
pub mod testing;

pub type Error = Box<dyn std::error::Error>;

pub async fn run() -> Result<(), Error> {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("❌ Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    dotfiles::first_run::first_run().await;

    match config.command.as_str() {
        "new" => project::new_project().await,
        "init" => project::init_project().await,
        "generate" => testing::generate::generate_testcases().await,
        "test" => testing::test::test_package().await,
        _ => println!("❌ I don't know what to do with {}!", config.command),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        let args = vec!["test".to_string(), "new".to_string()];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.command, "new");
    }

    #[test]
    fn config_error() {
        let args = vec!["test".to_string()];
        let config = Config::new(&args);
        assert!(config.is_err());
    }

    #[test]
    fn global_config() {
        let cfg = dotfiles::config::GlobalConfig::new();
        assert_eq!(cfg.template_branch, "main");
    }

    #[test]
    fn test_config() {
        let cfg = testing::config::TestConfig::default();
        assert_eq!(cfg.count as usize, cfg.tests.len());
    }
}
