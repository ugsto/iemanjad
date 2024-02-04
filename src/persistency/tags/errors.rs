use thiserror::Error;

#[derive(Error, Debug)]
pub enum TagRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),

    #[error("Failed to create tag in the database")]
    TagCreation,

    #[error("Failed to list tags from the database")]
    TagListing,

    #[error("Failed to count tags in the database")]
    TagCount,

    #[error("Failed to find tags by names in the database")]
    TagFind,
}
