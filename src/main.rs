use actix_web::{web, App, HttpServer};
use config::{
    models::{ApiBind, Config},
    strategies::{cli_config_loader::CliConfigLoader, env_config_loader::EnvConfigLoader},
    traits::PartialConfigLoader,
};
use persistency::{
    posts::surrealdb_posts_repository::SurrealdbPostsRepository,
    tags::surrealdb_tags_repository::SurrealdbTagsRepository,
    traits::{PostRepository, TagRepository},
};
use std::{io, process::exit};
use surrealdb::Surreal;

mod config;
mod handlers;
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
async fn main() -> io::Result<()> {
    let config = load_config();
    let db = load_db_connection(&config.db_address).await;

    let (post_repository, tag_repository) = create_repositories(db).await;

    let server = HttpServer::new(move || {
        let post_repository = post_repository.clone();
        let tag_repository = tag_repository.clone();

        App::new()
            .app_data(web::Data::new(post_repository))
            .app_data(web::Data::new(tag_repository))
            .service(
                web::resource("/api/v1/posts")
                    .route(web::post().to(handlers::posts::create_post::<
                        SurrealdbPostsRepository<SurrealdbTagsRepository>,
                    >))
                    .route(web::get().to(handlers::posts::find_all_posts::<
                        SurrealdbPostsRepository<SurrealdbTagsRepository>,
                    >)),
            )
            .service(
                web::resource("/api/v1/tags")
                    .route(web::get().to(handlers::tags::find_all_tags::<SurrealdbTagsRepository>))
                    .route(web::post().to(handlers::tags::create_tag::<SurrealdbTagsRepository>)),
            )
    });

    let server = match config.api_bind {
        ApiBind::UnixSocket(path) => server.bind_uds(path)?,
        ApiBind::Tcp(address) => server.bind(address)?,
    };

    server.run().await?;

    Ok(())
}
