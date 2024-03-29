use crate::{
    config::models::ApiBind,
    handlers,
    persistency::traits::{PostRepository, TagRepository},
};
use actix_web::{
    dev::{Service, ServiceRequest},
    web, App, HttpServer,
};
use tracing::info;

fn log_request(req: &ServiceRequest) {
    let method = req.method();
    let uri_path = req.uri().path();
    let peer_addr = req
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or("UNKNOWN CLIENT".to_string());

    info!("{method} {uri_path} | {peer_addr}");
}

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
            .wrap_fn(|req, srv| {
                log_request(&req);
                srv.call(req)
            })
            .app_data(web::Data::new(post_repository))
            .app_data(web::Data::new(tag_repository))
            .service(
                web::resource("/api/v1/posts")
                    .route(web::post().to(handlers::posts::create_post::<PR>))
                    .route(web::get().to(handlers::posts::find_all_posts::<PR>)),
            )
            .service(
                web::resource("/api/v1/posts/{id}")
                    .route(web::get().to(handlers::posts::get_post::<PR>))
                    .route(web::put().to(handlers::posts::update_post::<PR>))
                    .route(web::delete().to(handlers::posts::delete_post::<PR>)),
            )
            .service(
                web::resource("/api/v1/tags")
                    .route(web::get().to(handlers::tags::find_all_tags::<TR>))
                    .route(web::post().to(handlers::tags::create_tag::<TR>)),
            )
            .service(
                web::resource("/api/v1/tags/{name}")
                    .route(web::get().to(handlers::tags::get_tag::<TR>))
                    .route(web::put().to(handlers::tags::update_tag::<TR>))
                    .route(web::delete().to(handlers::tags::delete_tag::<TR>)),
            )
    });

    let server = match api_bind {
        ApiBind::UnixSocket(path) => server.bind_uds(path)?,
        ApiBind::Tcp(address) => server.bind(address)?,
    };

    server.run().await?;

    Ok(())
}
