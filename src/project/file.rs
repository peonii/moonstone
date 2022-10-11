use std::path::{PathBuf, Path};

use crate::Error;

pub struct Project {
    path: PathBuf
}

impl Project {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf()
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        // create the project directory if it doesn't exist
        if self.path.exists() {
            return Err("Project directory already exists".into());
        }
        
        std::fs::create_dir_all(&self.path)?;
        Ok(())
    }
}