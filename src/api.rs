use crate::{
    config::models::ApiBind,
    handlers,
    persistency::traits::{PostRepository, TagRepository},
};
use actix_web::{web, App, HttpServer};

pub async fn initialize_api<
    PR: PostRepository + Clone + Send + 'static,
    TR: TagRepository + Clone + Send + 'static,
>(
    (post_repository, tag_repository): (PR, TR),
    api_bind: ApiBind,
) -> anyhow::Result<()> {
    let server = HttpServer::new(move || {
        let post_repository = post_repository.clone();
        let tag_repository = tag_repository.clone();

        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(web::Data::new(post_repository))
            .app_data(web::Data::new(tag_repository))
            .service(
                web::resource("/api/v1/posts")
                    .route(web::post().to(handlers::posts::create_post::<PR>))
                    .route(web::get().to(handlers::posts::find_all_posts::<PR>)),
            )
            .service(
                web::resource("/api/v1/tags")
                    .route(web::get().to(handlers::tags::find_all_tags::<TR>))
                    .route(web::post().to(handlers::tags::create_tag::<TR>)),
            )
    });

    let server = match api_bind {
        ApiBind::UnixSocket(path) => server.bind_uds(path)?,
        ApiBind::Tcp(address) => server.bind(address)?,
    };

    server.run().await?;

    Ok(())
}
