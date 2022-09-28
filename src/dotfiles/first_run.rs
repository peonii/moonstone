use crate::dotfiles::config::GlobalConfig;
use colored::Colorize;

pub async fn first_run() {
    // does the config file exist?
    let home_dir = home::home_dir().unwrap();
    let mst_dir = home_dir.join(".moonstone.yml");

    if !mst_dir.exists() {
        println!("It seems like this is your first time using Moonstone!");
        println!("Allow me to help you set up your environment.");
        let config = GlobalConfig::new();
        GlobalConfig::save(&config);
        println!(
            "You can change the configuration later in {}!",
            "~/.moonstone.yml".yellow()
        );
    }
}
