pub struct Config {
    pub command: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();

        Ok(Config { command })
    }
}