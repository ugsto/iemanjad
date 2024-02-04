use super::models::{
    FindPostsResponse, NewPost, SurrealPostEntityInput, SurrealPostEntityOutput,
    SurrealPostEntityWithTagsOutput,
};
use crate::{
    models::Post,
    persistency::{
        models::{FindAllOptions, SurrealCountRecord},
        traits::{PostRepository, PostsTagsRepository, TagRepository},
    },
    utils::tag::{tags_diff_set, tags_to_ids},
};
use surrealdb::{engine::local::Db, Surreal};

#[derive(Clone)]
pub struct SurrealdbPostsRepository<TR: TagRepository, PTR: PostsTagsRepository> {
    db: Surreal<Db>,
    tags_repository: TR,
    posts_tags_repository: PTR,
}

impl<TR: TagRepository, PTR: PostsTagsRepository> SurrealdbPostsRepository<TR, PTR> {
    pub fn new(db: Surreal<Db>, tags_repository: TR, posts_tags_repository: PTR) -> Self {
        Self {
            db,
            tags_repository,
            posts_tags_repository,
        }
    }
}

impl<TR: TagRepository, PTR: PostsTagsRepository> PostRepository
    for SurrealdbPostsRepository<TR, PTR>
{
    async fn create(&self, new_post: NewPost) -> anyhow::Result<Post> {
        let tags = self
            .tags_repository
            .find_in_names(new_post.tags.iter().map(|tag| tag.as_str()).collect())
            .await?;

        if tags.len() != new_post.tags.len() {
            let diff = tags_diff_set(tags, &new_post.tags);

            return Err(anyhow::anyhow!("Tags not found: {:?}", diff));
        }

        let now = chrono::Utc::now();

        let post = SurrealPostEntityInput {
            title: new_post.title,
            content: new_post.content,
            created_at: surrealdb::sql::Datetime(now),
            updated_at: surrealdb::sql::Datetime(now),
        };

        let created_post = self
            .db
            .query("SELECT *, string::split(<string>id, ':')[1] AS id FROM (CREATE posts CONTENT $post);")
            .bind(("post", post))
            .await?
            .take::<Vec<SurrealPostEntityOutput>>(0)?
            .first()
            .ok_or(anyhow::anyhow!("Failed to create post"))?
            .clone();

        let tag_ids = tags_to_ids(tags.clone());

        self.posts_tags_repository
            .relate(
                &created_post.id,
                tag_ids.iter().map(|id| id.as_str()).collect(),
            )
            .await?;

        Ok((created_post, tags).into())
    }

    async fn find_all(&self, options: FindAllOptions) -> anyhow::Result<FindPostsResponse> {
        let posts = self
            .db
            .query("SELECT *, string::split(<string>id, ':')[1] AS id, (SELECT *, string::split(<string>id, ':')[1] AS id FROM ->posts_tags->tags.*) AS tags FROM posts LIMIT $limit START $offset")
            .bind(("limit", options.limit))
            .bind(("offset", options.offset))
            .await?
            .take::<Vec<SurrealPostEntityWithTagsOutput>>(0)?
            .into_iter()
            .map(|post| post.into())
            .collect();

        let total = self
            .db
            .query("SELECT COUNT(id) FROM posts GROUP ALL")
            .await?
            .take::<Vec<SurrealCountRecord>>(0)?
            .first()
            .unwrap_or_default()
            .count;

        Ok(FindPostsResponse { posts, total })
    }
}
