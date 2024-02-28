use actix_web::web;
use crate::config::db::Pool;
use crate::controller::book_controller::PageData;
use crate::model::book::{Book, NewBook, Page};
use crate::model::custom_errors::MyError;
use crate::model::dtos::FullBookDTO;

pub fn get_book_with_author(id: i32, pool: &web::Data<Pool>) -> Result<FullBookDTO, MyError> {
    Book::get_with_author(id, &mut pool.get().unwrap())
}

pub fn get_book_with_pages(id: i32, pool: &web::Data<Pool>) -> (Book, Vec<Page>) {
    Book::with_pages(id, &mut pool.get().unwrap())
}

pub fn get_book_list(limit: i64, pool: &web::Data<Pool>) -> Vec<(Book, Vec<Page>)> {
    Book::list_with_pages(limit, &mut pool.get().unwrap())
}

pub fn create_full_book(new_book: NewBook, pages: &Vec<PageData>, pool: &web::Data<Pool>)
    -> (Book, Vec<Page>) {
    Book::create_full_book(new_book, pages, &mut pool.get().unwrap())
}