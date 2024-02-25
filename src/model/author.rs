use diesel::{prelude::*, Insertable, Queryable};
use crate::schema::authors::{
    self,
    dsl::authors as author_table,
};
use serde::{Serialize, Deserialize};
use actix_web::{Responder, HttpResponse, error,
    http::{header::ContentType, StatusCode},
    body::BoxBody};
use dto_mapper::DtoMapper;
use super::custom_errors::MyError;

#[derive(Queryable, Identifiable, Selectable, Serialize, DtoMapper, Default, Clone)]
#[diesel(table_name = authors)]
#[mapper(dto = "AuthorDTO", ignore=["id"], derive=(Debug, Clone, PartialEq, Serialize))]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String,
}

impl Responder for Author {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(&self)
    }
}

impl Author {

    pub fn create_author(new_author: NewAuthor, conn: &mut PgConnection) -> Result<Author, MyError> {
        let result = new_author.insert_into(author_table).get_result(conn);
        match result {
            Ok(author) => Ok(author),
            Err(_err) =>  Err(MyError::InternalError),     
        }
    }
}