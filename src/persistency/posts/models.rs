use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::models::{Post, Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub tags: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealPostEntityInput {
    pub title: String,
    pub content: String,
    pub created_at: surrealdb::sql::Datetime,
    pub updated_at: surrealdb::sql::Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealPostEntityOutput {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: surrealdb::sql::Datetime,
    pub updated_at: surrealdb::sql::Datetime,
}

impl From<(SurrealPostEntityOutput, Vec<Tag>)> for Post {
    fn from((post, tags): (SurrealPostEntityOutput, Vec<Tag>)) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            tags,
            created_at: post.created_at.0,
            updated_at: post.updated_at.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealPostEntityWithTagsOutput {
    pub id: String,
    pub title: String,
    pub content: String,
    tags: Vec<Tag>,
    pub created_at: surrealdb::sql::Datetime,
    pub updated_at: surrealdb::sql::Datetime,
}

impl From<SurrealPostEntityWithTagsOutput> for Post {
    fn from(post: SurrealPostEntityWithTagsOutput) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            tags: post.tags,
            created_at: post.created_at.0,
            updated_at: post.updated_at.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindPostsResponse {
    pub posts: Vec<Post>,
    pub total: usize,
}
