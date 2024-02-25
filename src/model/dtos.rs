use serde::Serialize;

use super::book::BookDTO;
use super::author::AuthorDTO;
use actix_web::{Responder, HttpResponse, body::BoxBody};

#[derive(Serialize)]
pub struct FullBookDTO {
    pub book: BookDTO,
    pub author: AuthorDTO
}

impl Responder for FullBookDTO {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(&self)
    }
}