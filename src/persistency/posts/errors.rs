use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostRepositoryError {
    #[error("Database query failed: {0}")]
    Database(#[from] Box<dyn std::error::Error>),

    #[error("Failed to create post in the database")]
    PostCreation,

    #[error("Tags not found: {0:?}")]
    TagsNotFound(Vec<String>),

    #[error("Failed to list posts from the database")]
    PostListing,

    #[error("Failed to count posts in the database")]
    PostCount,

    #[error("Failed to update post in the database")]
    PostUpdate,
}
