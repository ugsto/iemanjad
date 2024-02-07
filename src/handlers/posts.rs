use crate::persistency::{models::FindAllOptions, posts::models::NewPost, traits::PostRepository};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn create_post<T: PostRepository>(
    post_repo: web::Data<T>,
    post: web::Json<NewPost>,
) -> impl Responder {
    match post_repo.create(post.into_inner()).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub async fn find_all_posts<T: PostRepository>(
    post_repo: web::Data<T>,
    query: web::Query<FindAllOptions>,
) -> impl Responder {
    match post_repo.find_all(query.into_inner()).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub async fn update_post<T: PostRepository>(
    post_repo: web::Data<T>,
    id: web::Path<String>,
    post: web::Json<NewPost>,
) -> impl Responder {
    match post_repo
        .update(id.into_inner().as_str(), post.into_inner())
        .await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub async fn delete_post<T: PostRepository>(
    post_repo: web::Data<T>,
    id: web::Path<String>,
) -> impl Responder {
    match post_repo.delete(id.into_inner().as_str()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}
