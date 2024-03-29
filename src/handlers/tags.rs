use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::persistency::{models::FindAllOptions, tags::models::NewTag, traits::TagRepository};

pub async fn create_tag<T: TagRepository>(
    tag_repo: web::Data<T>,
    tag: web::Json<NewTag>,
) -> impl Responder {
    match tag_repo.create(tag.into_inner()).await {
        Ok(tag) => HttpResponse::Created().json(tag),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub async fn find_all_tags<T: TagRepository>(
    tag_repo: web::Data<T>,
    query: web::Query<FindAllOptions>,
) -> impl Responder {
    match tag_repo.find_all(query.into_inner()).await {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub async fn get_tag<T: TagRepository>(
    tag_repo: web::Data<T>,
    name: web::Path<String>,
) -> impl Responder {
    match tag_repo.get(name.into_inner().as_str()).await {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub async fn update_tag<T: TagRepository>(
    tag_repo: web::Data<T>,
    name: web::Path<String>,
    tag: web::Json<NewTag>,
) -> impl Responder {
    match tag_repo
        .update(name.into_inner().as_str(), tag.into_inner())
        .await
    {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub async fn delete_tag<T: TagRepository>(
    tag_repo: web::Data<T>,
    name: web::Path<String>,
) -> impl Responder {
    match tag_repo.delete(name.into_inner().as_str()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}
