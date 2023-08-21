use diesel::{pg::Pg, prelude::*, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::posts::{
    self,
    dsl::*,
    published,
    table
};



#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

impl Post {

    pub fn first_five_published(connection: &mut PgConnection) -> Vec<Post> {

        posts
            .filter(published.eq(true))
            .limit(5)
            .select(Post::as_select())
            .load(connection)
            .expect("Error loading posts")
    }

    pub fn insert(new_post : &NewPost, connection: &mut PgConnection) -> Post {

        diesel::insert_into(table)
            .values(new_post)
            .returning(Post::as_returning())
            .get_result(connection)
            .expect("Error saving new post")
    }

    pub fn publish(serial: i32, connection: &mut PgConnection) -> Post {

        diesel::update(posts.find(serial))
            .set(published.eq(true))
            .returning(Post::as_returning())
            .get_result(connection)
            .expect("Error updating post")
    }

    pub fn delete(serial: i32, connection: &mut PgConnection) -> usize {

        diesel::delete(posts.find(serial))
            .execute(connection)
            .expect("Error deleting post")
    }
}