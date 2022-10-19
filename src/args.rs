use std::time::Instant;

use clap::{Parser, Subcommand};

use crate::{cache, project::{self, generation}, testing, Error};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        name: String,
    },
    Init,
    Generate {
        name: String,
        amount: u32,

        #[arg(default_value_t = 5000, short, long)]
        time_limit: u128,
    },
    Test {
        name: String,
    },
    ResetCache,
    Run
}

///
/// Parses the command line arguments and executes the appropriate command
///
pub async fn match_command() -> Result<(), Error> {
    let cli = Cli::parse();

    let timer = Instant::now();

    let result = match cli.command {
        Commands::New { name } => generation::new_project(Some(&name)),
        Commands::Init => generation::new_project(None),
        Commands::Generate {
            name,
            amount,
            time_limit,
        } => testing::generation::generate_tests(name, amount, time_limit).await,
        Commands::Test { name } => testing::test::test_package(name).await,
        Commands::ResetCache => cache::reset::reset_cache(),
        Commands::Run => project::run::run_project()
    };

    println!("✨ Done in {}s", timer.elapsed().as_secs_f64());

    result
}
