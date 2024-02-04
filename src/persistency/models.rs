use serde::{Deserialize, Serialize};

fn default_limit() -> usize {
    10
}
fn default_offset() -> usize {
    0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindAllOptions {
    #[serde(default = "default_limit")]
    pub limit: usize,

    #[serde(default = "default_offset")]
    pub offset: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealCountRecord {
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealRecord {
    pub id: String,
}

impl Default for &SurrealCountRecord {
    fn default() -> Self {
        &SurrealCountRecord { count: 0 }
    }
}
