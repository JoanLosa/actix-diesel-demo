use actix_web::web::{self,scope};
use crate::controller::book_controller::*;
use crate::controller::post_controller::*;
use crate::controller::author_controller::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {

    cfg.service(scope("/posts")
        .service(get_posts)
        .service(create_posts)
        .service(publish_post)
        .service(delete_post)
    ).service(scope("/books")
        .service(get_book_by_id)
        .service(get_books)
        .service(create_books)
        .service(insert_book)
    ).service(scope("/authors")
        .service(create_author));
}