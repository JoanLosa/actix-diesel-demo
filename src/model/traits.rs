use actix_web::Responder;
use super::custom_errors::MyError;
use diesel::prelude::PgConnection;

pub trait Saveable<R: Responder> {
    
    fn save(self, conn: &mut PgConnection) -> Result<R, MyError>;
}