use diesel::{prelude::*, Insertable, Queryable};
use diesel::result::Error::{DatabaseError};
use crate::schema::authors::{
    self,
    dsl::authors as author_table,
};
use serde::{Serialize, Deserialize};
use actix_web::{Responder, HttpResponse, error,
    http::{header::ContentType, StatusCode},
    body::BoxBody};
use derive_more::{Display, Error};

#[derive(Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String,
}

#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
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
            Err(_err) =>  Err(MyError::BadClientData),     
        }
    }
}