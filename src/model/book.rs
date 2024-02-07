use diesel::{prelude::*, Insertable, Queryable};
use crate::schema::{books::{
    self,
    dsl::*
}, pages::{
    self as my_pages,
    dsl::*
}};
use serde::{Serialize, Deserialize};
use crate::controller::book_controller::PageData;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Debug, PartialEq)]
#[diesel(table_name = books)]
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

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(Book))]
#[diesel(table_name = my_pages)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}

impl Book {

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