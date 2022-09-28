use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalConfig {
    pub template_repo: String,
    pub template_branch: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            template_repo: "peonii/oisuite-files".to_string(),
            template_branch: "main".to_string(),
        }
    }
}

impl GlobalConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save(config: &GlobalConfig) {
        let config = serde_yaml::to_string(&config).unwrap();

        let home_dir = home::home_dir().unwrap();
        let mst_dir = home_dir.join(".moonstone.yml");
        std::fs::write(mst_dir, config).unwrap();
    }

    pub fn load() -> Self {
        let home_dir = home::home_dir().unwrap();
        let mst_dir = home_dir.join(".moonstone.yml");

        if !mst_dir.exists() {
            let config = Self::new();
            GlobalConfig::save(&config);
        }

        let config = std::fs::read_to_string(mst_dir).unwrap();
        serde_yaml::from_str(&config).unwrap()
    }
}
