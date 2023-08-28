

use diesel::prelude::*;
use crate::viewmodels::NewPost;
use super::connection::establish_connection;

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> usize {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}
