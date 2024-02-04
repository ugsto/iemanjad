use surrealdb::{engine::local::Db, Surreal};

use super::models::SurrealTagsEntityOutput;
use crate::{models::Tag, persistency::traits::PostsTagsRepository};

#[derive(Clone)]
pub struct SurrealdbPostsTagsRepository {
    db: Surreal<Db>,
}

impl SurrealdbPostsTagsRepository {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }
}

impl PostsTagsRepository for SurrealdbPostsTagsRepository {
    async fn relate(&self, post_id: &str, tag_id: Vec<&str>) -> anyhow::Result<()> {
        let queries = tag_id
            .iter()
            .map(|tag_id| format!("RELATE posts:{}->posts_tags->tags:{}", post_id, tag_id))
            .collect::<Vec<_>>();

        self.db.query(queries.join(";")).await?;

        Ok(())
    }

    async fn find_tags_by_post_id(&self, post_id: &str) -> anyhow::Result<Vec<Tag>> {
        let tags = self
            .db
            .query("SELECT ->posts_tags->tags.* AS tags FROM posts:$post_id")
            .bind(("post_id", post_id))
            .await?
            .take::<Vec<SurrealTagsEntityOutput>>(0)?
            .first()
            .ok_or(anyhow::anyhow!("Tags not found"))?
            .clone()
            .tags;
        let tags = tags.into_iter().map(|tag| tag.into()).collect();

        Ok(tags)
    }
}
