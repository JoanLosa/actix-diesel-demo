use diesel::{prelude::*, Insertable, Queryable};
use diesel::result::Error::DatabaseError;
use crate::schema::authors::{
    self,
    dsl::authors as author_table,
};
use serde::{Serialize, Deserialize};

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

impl Author {

    pub fn create_author(new_author: NewAuthor, conn: &mut PgConnection) -> Result<Author, String> {

        match new_author.insert_into(author_table).get_result(conn) {
            Ok(author) => Ok(author),
            Err(err) => match err {
                DatabaseError(_kind, info) =>
                    Err(info.message().to_owned()),
                _ => Err(format!("unknown error"))
            }
        }
    }
}