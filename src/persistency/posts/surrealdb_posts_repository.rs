use super::{
    errors::PostRepositoryError,
    models::{
        FindPostsResponse, NewPost, SurrealPostEntityInput, SurrealPostEntityOutput,
        SurrealPostEntityWithTagsOutput,
    },
    utils::create_post_entity,
};
use crate::{
    models::{Post, Tag},
    persistency::{
        models::{FindAllOptions, SurrealCountRecord},
        traits::{PostRepository, TagRepository},
    },
    utils::tag::tags_diff_set,
};
use surrealdb::Surreal;
use tracing::{debug, info};

#[derive(Clone)]
pub struct SurrealdbPostsRepository<TR: TagRepository> {
    db: Surreal<surrealdb::engine::any::Any>,
    tags_repository: TR,
}

impl<TR: TagRepository> SurrealdbPostsRepository<TR> {
    pub fn new(db: Surreal<surrealdb::engine::any::Any>, tags_repository: TR) -> Self {
        Self {
            db,
            tags_repository,
        }
    }
}

impl<TR: TagRepository> SurrealdbPostsRepository<TR> {
    async fn register_post_in_db(
        &self,
        post_entity: SurrealPostEntityInput,
    ) -> Result<SurrealPostEntityOutput, PostRepositoryError> {
        debug!("Creating post...");

        let result = self
            .db
            .query(include_str!("./queries/create_post.surql"))
            .bind(("post", post_entity))
            .await;

        debug!("Created post: {result:?}");

        let post = result
            .map_err(|e| PostRepositoryError::Database(e.into()))?
            .take::<Vec<SurrealPostEntityOutput>>(0)
            .map_err(|_| PostRepositoryError::PostCreation)?
            .first()
            .cloned()
            .ok_or(PostRepositoryError::PostCreation)?;

        info!("Created post: {post:?}");

        Ok(post)
    }

    async fn sync_relations_in_db(
        &self,
        post_id: &str,
        tags: &[Tag],
    ) -> Result<(), PostRepositoryError> {
        debug!("Syncing relations for post {post_id} with tags {tags:?}...");

        let post_id = format!("posts:{}", post_id);
        let tag_ids = tags
            .iter()
            .map(|tag| format!("tags:{}", tag.id))
            .collect::<Vec<_>>();

        debug!("Syncing relations for post {post_id} with tags {tag_ids:?}...");

        let response = self
            .db
            .query(include_str!("./queries/sync_relations.surql"))
            .bind(("post_id", post_id.as_str()))
            .bind(("tag_ids", &tag_ids))
            .await
            .map_err(|e| PostRepositoryError::Database(e.into()))?;

        info!("Synced relations for post {post_id} with tags {tag_ids:?}: {response:?}");

        Ok(())
    }

    async fn list_posts_in_db(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<SurrealPostEntityWithTagsOutput>, PostRepositoryError> {
        debug!("Listing posts...");

        let result = self
            .db
            .query(include_str!("./queries/list_posts.surql"))
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await;

        debug!("Listed posts: {result:?}");

        let posts = result
            .map_err(|e| PostRepositoryError::Database(e.into()))?
            .take::<Vec<SurrealPostEntityWithTagsOutput>>(0)
            .map_err(|_| PostRepositoryError::PostListing)?;

        info!("Listed posts: {posts:?}");

        Ok(posts)
    }

    async fn count_posts_in_db(&self) -> Result<usize, PostRepositoryError> {
        debug!("Counting posts...");

        let result = self
            .db
            .query(include_str!("./queries/count_posts.surql"))
            .await;

        debug!("Counted posts: {result:?}");

        let total = result
            .map_err(|e| PostRepositoryError::Database(e.into()))?
            .take::<Vec<SurrealCountRecord>>(0)
            .map_err(|_| PostRepositoryError::PostCount)?
            .first()
            .unwrap_or_default()
            .count;

        info!("Counted posts: {total}");

        Ok(total)
    }

    async fn get_post_in_db(
        &self,
        post_id: &str,
    ) -> Result<SurrealPostEntityWithTagsOutput, PostRepositoryError> {
        let post_id = format!("posts:{post_id}");

        debug!("Fetching post {post_id}...");

        let result = self
            .db
            .query(include_str!("./queries/get_post.surql"))
            .bind(("post_id", post_id.as_str()))
            .await;

        debug!("Fetched post: {result:?}");

        let post = result
            .map_err(|e| PostRepositoryError::Database(e.into()))?
            .take::<Vec<SurrealPostEntityWithTagsOutput>>(0)
            .map_err(|_| PostRepositoryError::PostGet)?
            .first()
            .cloned()
            .ok_or(PostRepositoryError::PostGet)?;

        info!("Fetched post: {post:?}");

        Ok(post)
    }

    async fn update_post_in_db(
        &self,
        post_id: &str,
        post_entity: &SurrealPostEntityInput,
    ) -> Result<SurrealPostEntityOutput, PostRepositoryError> {
        let post_id = format!("posts:{post_id}");

        debug!("Updating post {post_id}: {post_entity:?}...");

        let response = self
            .db
            .query(include_str!("./queries/update_posts.surql"))
            .bind(("id", post_id.as_str()))
            .bind(("title", post_entity.title.as_str()))
            .bind(("content", post_entity.content.as_str()))
            .await;

        debug!("Updated post {post_id}: {response:?}");

        let post = response
            .map_err(|e| PostRepositoryError::Database(e.into()))?
            .take::<Vec<SurrealPostEntityOutput>>(0)
            .map_err(|_| PostRepositoryError::PostUpdate)?
            .first()
            .cloned()
            .ok_or(PostRepositoryError::PostUpdate)?;

        debug!("Updated post {post_id}: {post:?}");

        Ok(post)
    }

    async fn delete_post_in_db(&self, post_id: &str) -> Result<(), PostRepositoryError> {
        let post_id = format!("posts:{post_id}");

        debug!("Deleting post {post_id}...");

        let response = self
            .db
            .query(include_str!("./queries/delete_post.surql"))
            .bind(("post_id", post_id.as_str()))
            .await;

        debug!("Deleted post {post_id}: {response:?}");

        response.map_err(|e| PostRepositoryError::Database(e.into()))?;

        info!("Deleted post {post_id}");

        Ok(())
    }
}

impl<TR: TagRepository> PostRepository for SurrealdbPostsRepository<TR> {
    async fn create(&self, new_post: NewPost) -> Result<Post, PostRepositoryError> {
        let tags = self
            .tags_repository
            .find_in_names(new_post.tags.iter().map(|tag| tag.as_str()).collect())
            .await
            .map_err(|e| PostRepositoryError::Database(e.into()))?;

        if tags.len() != new_post.tags.len() {
            let diff = tags_diff_set(tags, &new_post.tags);
            return Err(PostRepositoryError::TagsNotFound(
                diff.into_iter().collect(),
            ));
        }

        let post_entity = create_post_entity(new_post.title, new_post.content, chrono::Utc::now());

        let created_post = self.register_post_in_db(post_entity).await?;
        self.sync_relations_in_db(&created_post.id, &tags).await?;

        Ok((created_post, tags).into())
    }

    async fn find_all(
        &self,
        options: FindAllOptions,
    ) -> Result<FindPostsResponse, PostRepositoryError> {
        let posts = self
            .list_posts_in_db(options.limit, options.offset)
            .await?
            .into_iter()
            .map(|post| post.into())
            .collect();

        let total = self.count_posts_in_db().await?;

        Ok(FindPostsResponse { posts, total })
    }

    async fn get(&self, id: &str) -> Result<Post, PostRepositoryError> {
        let post = self.get_post_in_db(id).await?;

        Ok(post.into())
    }

    async fn update(&self, id: &str, new_post: NewPost) -> Result<Post, PostRepositoryError> {
        let post_entity = create_post_entity(new_post.title, new_post.content, chrono::Utc::now());

        let tags = self
            .tags_repository
            .find_in_names(
                new_post
                    .tags
                    .iter()
                    .map(|tag| tag.as_str())
                    .collect::<Vec<_>>(),
            )
            .await
            .map_err(|_| PostRepositoryError::PostUpdate)?;
        let updated_post = self.update_post_in_db(id, &post_entity).await?;
        self.sync_relations_in_db(&updated_post.id, &tags).await?;

        Ok((updated_post, tags).into())
    }

    async fn delete(&self, id: &str) -> Result<(), PostRepositoryError> {
        self.delete_post_in_db(id).await?;

        Ok(())
    }
}
