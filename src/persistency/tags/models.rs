use crate::models::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealTagEntityInput {
    pub name: String,
}

impl From<NewTag> for SurrealTagEntityInput {
    fn from(tag: NewTag) -> Self {
        Self { name: tag.name }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealTagEntityOutput {
    pub id: String,
    pub name: String,
}

impl From<SurrealTagEntityOutput> for Tag {
    fn from(tag: SurrealTagEntityOutput) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindTagsResponse {
    pub tags: Vec<Tag>,
    pub total: usize,
}
