use super::{
    models::FindAllOptions,
    posts::{
        errors::PostRepositoryError,
        models::{FindPostsResponse, NewPost},
    },
    tags::{
        errors::TagRepositoryError,
        models::{FindTagsResponse, NewTag},
    },
};
use crate::models::{Post, Tag};

pub trait PostRepository {
    async fn create(&self, new_post: NewPost) -> Result<Post, PostRepositoryError>;
    async fn find_all(
        &self,
        options: FindAllOptions,
    ) -> Result<FindPostsResponse, PostRepositoryError>;
    async fn get(&self, id: &str) -> Result<Post, PostRepositoryError>;
    async fn update(&self, id: &str, new_post: NewPost) -> Result<Post, PostRepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), PostRepositoryError>;
}

pub trait TagRepository {
    async fn create(&self, new_tag: NewTag) -> Result<Tag, TagRepositoryError>;
    async fn find_all(
        &self,
        options: FindAllOptions,
    ) -> Result<FindTagsResponse, TagRepositoryError>;
    async fn find_in_names(&self, names: Vec<&str>) -> Result<Vec<Tag>, TagRepositoryError>;
    async fn get(&self, name: &str) -> Result<Tag, TagRepositoryError>;
    async fn update(&self, name: &str, new_tag: NewTag) -> Result<Tag, TagRepositoryError>;
    async fn delete(&self, name: &str) -> Result<(), TagRepositoryError>;
}
