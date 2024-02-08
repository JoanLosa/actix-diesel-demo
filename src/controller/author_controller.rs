use actix_web::{HttpResponse, post, web::{Data, Json}};
use crate::config::db::Pool;
use crate::model::author::{NewAuthor, Author};
use crate::service::author_service;

#[post("/")]
pub async fn create_author(request: Json<NewAuthor>, pool: Data<Pool>) -> HttpResponse {
    
    let result: Result<Author, String> =  author_service::create_author(request.0, &pool);
    match result {
        Ok(author) => HttpResponse::Ok().json(author),
        Err(message) => HttpResponse::BadRequest().body(message)
    }
}