use api::initialize_api;
use config::{
    models::Config,
    strategies::{cli_config_loader::CliConfigLoader, env_config_loader::EnvConfigLoader},
    traits::PartialConfigLoader,
};
use logger::initialize_logger;
use migrations::{exec_migrations, MIGRATIONS};
use persistency::{
    posts::surrealdb_posts_repository::SurrealdbPostsRepository,
    tags::surrealdb_tags_repository::SurrealdbTagsRepository,
    traits::{PostRepository, TagRepository},
};
use std::process::exit;
use surrealdb::Surreal;
use tracing::{debug, info};

mod api;
mod config;
mod handlers;
mod logger;
mod migrations;
mod models;
mod persistency;
mod utils;

fn load_config() -> Config {
    let env_config = EnvConfigLoader::load_partial_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config from environment: {e}");
        Default::default()
    });
    let cli_config = CliConfigLoader::load_partial_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config from CLI: {e}");
        Default::default()
    });

    env_config.merge(cli_config).try_into().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {e}");
        exit(1);
    })
}

async fn load_db_connection(address: &str) -> Surreal<surrealdb::engine::any::Any> {
    let db = surrealdb::engine::any::connect(address).await.unwrap();
    db.use_ns("iemanjad").use_db("posts").await.unwrap();

    db
}

async fn create_repositories(
    db: Surreal<surrealdb::engine::any::Any>,
) -> (impl PostRepository + Clone, impl TagRepository + Clone) {
    (
        SurrealdbPostsRepository::new(db.clone(), SurrealdbTagsRepository::new(db.clone())),
        SurrealdbTagsRepository::new(db.clone()),
    )
}

#[actix_web::main]
async fn main() {
    let config = load_config();

    initialize_logger(&config.log_level);
    debug!(?config);

    debug!("Connecting to database...");
    let db = load_db_connection(&config.db_address).await;
    debug!("Database connected");

    // TODO: Find more elegant way to do this
    exec_migrations(&db, MIGRATIONS).await;

    debug!("Loading repositories...");
    let (post_repository, tag_repository) = create_repositories(db).await;
    debug!("Repositories loaded");

    info!("Starting server on {:?}", config.api_bind);
    initialize_api((post_repository, tag_repository), config.api_bind)
        .await
        .unwrap();

    info!("Shutting down...");
}
