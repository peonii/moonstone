use clap::{Parser, Subcommand};

use crate::{Error, project::generation, testing};

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
        time_limit: u32,
        
        #[arg(default_value_t = 64, short, long)]
        memory_limit: u32,
    },
    Test {
        name: String
    }
}

///
/// Parses the command line arguments and executes the appropriate command
/// 
pub async fn match_command() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => {
            generation::new_project(Some(&name))
        }
        Commands::Init => {
            generation::new_project(None)
        }
        Commands::Generate { name, amount, time_limit, memory_limit } => {
            testing::generation::generate_tests(name, amount, time_limit, memory_limit).await
        }
        Commands::Test { name } => {
            testing::test::test_package(name).await
        }
    }
}