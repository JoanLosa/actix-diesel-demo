use actix_web::web;
use crate::config::db::Pool;
use crate::model::author::{Author, NewAuthor};

pub fn create_author(new_author: NewAuthor, pool: &web::Data<Pool>) -> Result<Author, String> {

    Author::create_author(new_author, &mut pool.get().unwrap())
}