use crate::config::Config;
use std::env;

mod config;
mod dotfiles;
mod project;
mod testing;

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
