use super::{
    models::FindAllOptions,
    posts::models::{FindPostsResponse, NewPost},
    tags::models::{FindTagsResponse, NewTag},
};
use crate::models::{Post, Tag};

pub trait PostRepository {
    async fn create(&self, new_post: NewPost) -> anyhow::Result<Post>;
    async fn find_all(&self, options: FindAllOptions) -> anyhow::Result<FindPostsResponse>;
}

pub trait TagRepository {
    async fn create(&self, new_tag: NewTag) -> anyhow::Result<Tag>;
    async fn find_all(&self, options: FindAllOptions) -> anyhow::Result<FindTagsResponse>;
    async fn find_in_names(&self, names: Vec<&str>) -> anyhow::Result<Vec<Tag>>;
}

pub trait PostsTagsRepository {
    async fn relate(&self, post_id: &str, tag_ids: Vec<&str>) -> anyhow::Result<()>;
    async fn find_tags_by_post_id(&self, post_id: &str) -> anyhow::Result<Vec<Tag>>;
}
