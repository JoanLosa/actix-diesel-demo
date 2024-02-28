use actix_web::{get, HttpResponse, post, web::{Path, Data, Json}};
use serde::Deserialize;
use crate::{config::db::Pool, model::{custom_errors::MyError, dtos::FullBookDTO}};
use crate::model::book::{Book, NewBook, Page};
use crate::service::book_service;


#[derive(Deserialize)]
pub struct BookID {
    id: i32
}

#[derive(Deserialize)]
pub struct BookParams {
    limit: i64
}

#[derive(Deserialize)]
pub struct PageData {
    pub page_number: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub struct FullBook {
    title: String,
    author_id: i32,
    pages: Vec<PageData>,
}

#[get("/{id}")]
pub async fn get_book_by_id(book_id: Path<BookID>, pool: Data<Pool>) -> Result<FullBookDTO, MyError> {    
    book_service::get_book_with_author(book_id.id, &pool)
}

#[get("/")]
pub async fn get_books(book_params: Path<BookParams>, pool: Data<Pool>) -> HttpResponse {
    let books: Vec<(Book, Vec<Page>)> = book_service::get_book_list(book_params.limit, &pool);
    HttpResponse::Ok().json(books)
}

#[post("/")]
pub async fn create_books(full_book: Json<FullBook>, pool: Data<Pool>) -> HttpResponse {

    let new_book: NewBook = NewBook {title: full_book.title.clone(), author_id: full_book.author_id};
    let result = book_service::create_full_book(new_book, &full_book.pages,&pool);
    HttpResponse::Ok().json(result)
}
