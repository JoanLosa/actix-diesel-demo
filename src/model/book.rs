use actix_web::{Responder, body::BoxBody, HttpResponse};
use diesel::{prelude::*, result::Error, Insertable, Queryable};
use crate::schema::{books::{
    self,
    dsl::*
}, pages::{
    self as my_pages,
    dsl::*
}, authors};
use serde::{Serialize, Deserialize};
use crate::controller::book_controller::PageData;
use dto_mapper::DtoMapper;
use super::{author::Author, custom_errors::MyError, dtos::FullBookDTO, traits::Saveable};

#[derive(Insertable, Deserialize)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Debug, PartialEq, DtoMapper, Default, Clone)]
#[diesel(table_name = books)]
#[diesel(belongs_to(Author))]
#[mapper(dto = "BookDTO", ignore=["author_id"], derive=(Debug, Clone, PartialEq, Serialize))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = my_pages)]
pub struct NewPage {
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, DtoMapper, Default, Clone)]
#[diesel(belongs_to(Book))]
#[diesel(table_name = my_pages)]
#[mapper(dto = "PageDTO", ignore=["id","book_id"], derive=(Debug, Clone, PartialEq, Serialize))]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}

impl Book {

    pub fn get_with_author(pk: i32, conn: &mut PgConnection) -> Result<FullBookDTO, MyError> {
        let book_with_author: Result<Option<(Book, Author)>, Error>  = books::table
            .inner_join(authors::table)
            .filter(authors::id.eq(books::author_id).and(books::id.eq(pk)))
            .first(conn)
            .optional();

        match book_with_author {
            Ok(option) => match option {
                Some((book, author)) => Ok(FullBookDTO {book: book.clone().into(),
                    author: author.clone().into()}),
                None => Err(MyError::NotHere),
            },
            Err(error) => Err(MyError::InternalError),
        }
    }

    pub fn with_pages(pk: i32, conn: &mut PgConnection) -> (Book, Vec<Page>) {

        let book: Book = books.find(pk).first(conn).expect("Error getting book");
        let res: Vec<Page> = Page::belonging_to(&book)
            .select(Page::as_select())
            .load(conn).expect("Error loading pages");
        (book,res)
    }

    pub fn list_with_pages(limit: i64, conn: &mut PgConnection) -> Vec<(Book, Vec<Page>)> {

        let book_list = books::table.select(Book::as_select()).limit(limit)
            .load(conn).expect("Failure retrieving books");

        let page_list = Page::belonging_to(&book_list)
            .select(Page::as_select())
            .load(conn).expect("failure retrieving pages");

        page_list
            .grouped_by(&book_list)
            .into_iter()
            .zip(book_list)
            .map(|(page, book)| (book, page))
            .collect::<Vec<(Book, Vec<Page>)>>()
    }

    pub fn create_full_book(new_book: NewBook, page_data: &Vec<PageData>, conn: &mut PgConnection)
                            -> (Book, Vec<Page>) {

        let book: Book = new_book.insert_into(books).get_result(conn).expect("Book expected");
        let mut page_vec: Vec<NewPage> = Vec::new();
        for data in page_data {
            page_vec.push(NewPage {page_number: data.page_number,
                content: data.content.clone(), book_id: book.id })
        }
        let res: Vec<Page> = page_vec.insert_into(pages).get_results(conn).expect("Pages");
        (book, res)
    }
}

impl Responder for BookDTO {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(&self)
    }
}

impl Saveable<BookDTO> for NewBook {

    fn save(self, conn: &mut PgConnection) -> Result<BookDTO, MyError> {
        let result: Result<Book, Error> = self.insert_into(books::table).get_result(conn);
        match result {
            Ok(r) => Ok(r.clone().into()),
            Err(_err) => Err(MyError::InternalError),
        }
    }
}

