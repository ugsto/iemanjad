use crate::persistency::tags::models::SurrealTagEntityOutput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealTagsEntityOutput {
    pub tags: Vec<SurrealTagEntityOutput>,
}
