use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestConfig {
    pub count: u32,
    pub time_limit: u32,
    pub memory_limit: u32,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            count: 100,
            time_limit: 1000,
            memory_limit: 256,
        }
    }
}

impl TestConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
