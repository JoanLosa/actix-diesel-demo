use actix_web::{post, web::{Data, Json}};
use crate::{config::db::Pool, model::custom_errors::MyError};
use crate::model::author::{NewAuthor, Author};
use crate::service::author_service;

#[post("/")]
pub async fn create_author(request: Json<NewAuthor>, pool: Data<Pool>) -> Result<Author, MyError> {    
    author_service::create_author(request.0, &pool)    
}