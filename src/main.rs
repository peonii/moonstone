use std::env;
use crate::config::Config;

mod config;
mod project;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    match config.command.as_str() {
        "new" => project::new_project(),
        "init" => project::init_project(),
        _ => println!("I don't know what to do with {}!", config.command),
    }
}
