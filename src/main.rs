use actix_web::{web, App, HttpServer};
use persistency::{
    posts::surrealdb_posts_repository::SurrealdbPostsRepository,
    tags::surrealdb_tags_repository::SurrealdbTagsRepository,
    traits::{PostRepository, TagRepository},
};
use std::io;
use surrealdb::{engine::local::SpeeDb, Surreal};

mod handlers;
mod models;
mod persistency;
mod utils;

async fn create_repositories() -> (impl PostRepository + Clone, impl TagRepository + Clone) {
    let db = Surreal::new::<SpeeDb>("/tmp/iemanjad.surreal")
        .await
        .unwrap();
    db.use_ns("iemanjad").use_db("posts").await.unwrap();

    (
        SurrealdbPostsRepository::new(db.clone(), SurrealdbTagsRepository::new(db.clone())),
        SurrealdbTagsRepository::new(db.clone()),
    )
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let unix_socket_path = "/tmp/actix-uds";
    let (post_repository, tag_repository) = create_repositories().await;

    HttpServer::new(move || {
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
