use super::{
    errors::TagRepositoryError,
    models::{FindTagsResponse, NewTag, SurrealTagEntityInput, SurrealTagEntityOutput},
};
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

impl SurrealdbTagsRepository {
    async fn register_tag_in_db(
        &self,
        tag_entity: SurrealTagEntityInput,
    ) -> Result<SurrealTagEntityOutput, TagRepositoryError> {
        let tag = self
            .db
            .query(include_str!("./queries/create_tag.surql"))
            .bind(("tag", tag_entity))
            .await
            .map_err(TagRepositoryError::Database)?
            .take::<Vec<SurrealTagEntityOutput>>(0)
            .map_err(|_| TagRepositoryError::TagCreation)?
            .first()
            .cloned()
            .ok_or(TagRepositoryError::TagCreation)?;

        Ok(tag)
    }

    async fn list_tags_in_db(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<SurrealTagEntityOutput>, TagRepositoryError> {
        let tags = self
            .db
            .query(include_str!("./queries/list_tags.surql"))
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await
            .map_err(TagRepositoryError::Database)?
            .take::<Vec<SurrealTagEntityOutput>>(0)
            .map_err(|_| TagRepositoryError::TagListing)?;

        Ok(tags)
    }

    async fn count_tags_in_db(&self) -> Result<usize, TagRepositoryError> {
        let total = self
            .db
            .query(include_str!("./queries/count_tags.surql"))
            .await
            .map_err(TagRepositoryError::Database)?
            .take::<Vec<SurrealCountRecord>>(0)
            .map_err(|_| TagRepositoryError::TagCount)?
            .first()
            .unwrap_or_default()
            .count;

        Ok(total)
    }

    async fn find_tags_by_names_in_db(
        &self,
        names: Vec<&str>,
    ) -> Result<Vec<SurrealTagEntityOutput>, TagRepositoryError> {
        let tags = self
            .db
            .query(include_str!("./queries/find_tags_by_names.surql"))
            .bind(("names", names))
            .await
            .map_err(TagRepositoryError::Database)?
            .take::<Vec<SurrealTagEntityOutput>>(0)
            .map_err(|_| TagRepositoryError::TagFind)?;

        Ok(tags)
    }
}

impl TagRepository for SurrealdbTagsRepository {
    async fn create(&self, new_tag: NewTag) -> Result<Tag, TagRepositoryError> {
        let tag_entity = SurrealTagEntityInput::from(new_tag);

        let created_tag = self.register_tag_in_db(tag_entity).await?.into();

        Ok(created_tag)
    }

    async fn find_all(
        &self,
        options: FindAllOptions,
    ) -> Result<FindTagsResponse, TagRepositoryError> {
        let tags = self
            .list_tags_in_db(options.limit, options.offset)
            .await?
            .into_iter()
            .map(|tag| tag.into())
            .collect();

        let total = self.count_tags_in_db().await?;

        Ok(FindTagsResponse { tags, total })
    }

    async fn find_in_names(&self, names: Vec<&str>) -> Result<Vec<Tag>, TagRepositoryError> {
        let tags = self
            .find_tags_by_names_in_db(names)
            .await?
            .into_iter()
            .map(|tag| tag.into())
            .collect();

        Ok(tags)
    }
}