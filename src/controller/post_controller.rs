use actix_web::{get, HttpResponse, post, web, patch, delete};
use serde::Deserialize;
use crate::config::db::Pool;
use crate::service::post_service::*;
use crate::model::post::{NewPost, Post};

#[derive(Deserialize)]
pub struct PostID {
    id: i32
}

#[get("/")]
pub async fn get_posts(pool: web::Data<Pool>) -> HttpResponse {

    let results = publish_list(&pool);
    HttpResponse::Ok().json(results)
}

#[post("/")]
pub async fn create_posts(info: web::Json<NewPost>, pool: web::Data<Pool>) -> HttpResponse {

    let post : Post = create(&info.0, &pool);
    HttpResponse::Ok().json(post)
}

#[patch("/{id}/publish")]
pub async fn publish_post(post_id: web::Path<PostID>, pool: web::Data<Pool>) -> HttpResponse {

    let result = publish(post_id.id, &pool);
    HttpResponse::Ok().json(result)
}

#[delete("/{id}")]
pub async fn delete_post(post_id : web::Path<PostID>, pool: web::Data<Pool>) -> HttpResponse {

    let result = delete(post_id.id, &pool);
    HttpResponse::Ok().json(format!("Deleted {} posts", result))
}