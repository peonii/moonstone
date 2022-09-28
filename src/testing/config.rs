use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestConfig {
    pub count: u32,
    pub time_limit: u32,
    pub memory_limit: u32,
    pub tests: Vec<Test>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    pub input: String,
    pub output: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            count: 0,
            time_limit: 1000,
            memory_limit: 256,
            tests: Vec::new(),
        }
    }
}
