use super::{
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
    utils::tag::{tags_diff_set, tags_to_ids},
};
use surrealdb::{engine::local::Db, Surreal};

#[derive(Clone)]
pub struct SurrealdbPostsRepository<TR: TagRepository> {
    db: Surreal<Db>,
    tags_repository: TR,
}

impl<TR: TagRepository> SurrealdbPostsRepository<TR> {
    pub fn new(db: Surreal<Db>, tags_repository: TR) -> Self {
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
    ) -> anyhow::Result<SurrealPostEntityOutput> {
        let post = self
            .db
            .query(include_str!("./queries/create_post.surql"))
            .bind(("post", post_entity))
            .await?
            .take::<Vec<SurrealPostEntityOutput>>(0)?
            .first()
            .ok_or(anyhow::anyhow!("Failed to create post"))
            .cloned()?;

        Ok(post)
    }

    async fn register_relations_in_db(&self, post_id: &str, tags: Vec<Tag>) -> anyhow::Result<()> {
        let tag_ids = tags_to_ids(tags.clone());
        let relate_queries = tag_ids
            .iter()
            .map(|tag_id| {
                format!(
                    include_str!("./queries/relate_post_to_tag.tmpl"),
                    tag_id = tag_id,
                    post_id = post_id
                )
            })
            .collect::<Vec<_>>();
        self.db.query(relate_queries.join(";")).await?;

        Ok(())
    }

    async fn list_posts_in_db(
        &self,
        limit: usize,
        offset: usize,
    ) -> anyhow::Result<Vec<SurrealPostEntityWithTagsOutput>> {
        let posts = self
            .db
            .query(include_str!("./queries/list_posts.surql"))
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await?
            .take::<Vec<SurrealPostEntityWithTagsOutput>>(0)?;

        Ok(posts)
    }

    async fn count_posts_in_db(&self) -> anyhow::Result<usize> {
        let total = self
            .db
            .query(include_str!("./queries/count_posts.surql"))
            .await?
            .take::<Vec<SurrealCountRecord>>(0)?
            .first()
            .unwrap_or_default()
            .count;

        Ok(total)
    }
}

impl<TR: TagRepository> PostRepository for SurrealdbPostsRepository<TR> {
    async fn create(&self, new_post: NewPost) -> anyhow::Result<Post> {
        let tags = self
            .tags_repository
            .find_in_names(new_post.tags.iter().map(|tag| tag.as_str()).collect())
            .await?;

        if tags.len() != new_post.tags.len() {
            let diff = tags_diff_set(tags, &new_post.tags);

            return Err(anyhow::anyhow!("Tags not found: {:?}", diff));
        }

        let post_entity = create_post_entity(new_post.title, new_post.content, chrono::Utc::now());

        let created_post = self.register_post_in_db(post_entity).await?;
        self.register_relations_in_db(&created_post.id, tags.clone())
            .await?;

        Ok((created_post, tags).into())
    }

    async fn find_all(&self, options: FindAllOptions) -> anyhow::Result<FindPostsResponse> {
        let posts = self
            .list_posts_in_db(options.limit, options.offset)
            .await?
            .into_iter()
            .map(|post| post.into())
            .collect();

        let total = self.count_posts_in_db().await?;

        Ok(FindPostsResponse { posts, total })
    }
}
