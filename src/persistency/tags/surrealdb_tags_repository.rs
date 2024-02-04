use super::models::{FindTagsResponse, NewTag, SurrealTagEntityInput, SurrealTagEntityOutput};
use crate::{
    models::Tag,
    persistency::{
        models::{FindAllOptions, SurrealCountRecord},
        traits::TagRepository,
    },
};
use surrealdb::{engine::local::Db, Surreal};

#[derive(Clone)]
pub struct SurrealdbTagsRepository {
    db: Surreal<Db>,
}

impl SurrealdbTagsRepository {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }
}

impl TagRepository for SurrealdbTagsRepository {
    async fn create(&self, new_tag: NewTag) -> anyhow::Result<Tag> {
        let tag = SurrealTagEntityInput::from(new_tag);

        let created_tag = self
            .db
            .query(
                "SELECT *, string::split(<string>id, ':')[1] AS id FROM (CREATE tags CONTENT $tag)",
            )
            .bind(("tag", tag))
            .await?
            .take::<Vec<SurrealTagEntityOutput>>(0)?
            .first()
            .ok_or(anyhow::anyhow!("Tag not created"))?
            .clone()
            .into();

        Ok(created_tag)
    }

    async fn find_all(&self, options: FindAllOptions) -> anyhow::Result<FindTagsResponse> {
        let tags = self
            .db
            .query("SELECT *, string::split(<string>id, ':')[1] AS id FROM tags LIMIT $limit START $offset")
            .bind(("limit", options.limit))
            .bind(("offset", options.offset))
            .await?
            .take(0)?;

        let total = self
            .db
            .query("SELECT COUNT(id) FROM tags GROUP ALL")
            .await?
            .take::<Vec<SurrealCountRecord>>(0)?
            .first()
            .unwrap_or_default()
            .count;

        Ok(FindTagsResponse { tags, total })
    }

    async fn find_in_names(&self, names: Vec<&str>) -> anyhow::Result<Vec<Tag>> {
        let tags = self
            .db
            .query(
                "SELECT *, string::split(<string>id, ':')[1] AS id FROM tags WHERE name IN $names",
            )
            .bind(("names", names))
            .await?
            .take(0)?;

        Ok(tags)
    }
}
