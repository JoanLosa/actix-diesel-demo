use actix_web::web;
use crate::config::db::Pool;
use crate::model::post::{NewPost, Post};

pub fn publish_list(pool: &web::Data<Pool>) -> Vec<Post> {

    Post::first_five_published(&mut pool.get().unwrap())
}

pub fn create(new_post: &NewPost, pool: &web::Data<Pool>) -> Post {

    Post::insert(new_post,&mut pool.get().unwrap())
}

pub fn publish(id: i32, pool: &web::Data<Pool>) -> Post {

    Post::publish(id, &mut pool.get().unwrap())
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> usize {

    Post::delete(id, &mut pool.get().unwrap())
}