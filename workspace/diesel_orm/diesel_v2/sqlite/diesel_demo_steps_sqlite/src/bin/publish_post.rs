/// > ` Binary `
/// ```cargo run -q -p diesel_demo_steps_sqlite --bin publish_post 2```
///
/// ```cargo doc  --package diesel_demo_steps_sqlite --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package diesel_demo_steps_sqlite```
///
use diesel::prelude::*;
use std::env::args;
use diesel::prelude::*;
use diesel_demo_steps_sqlite::models::*;
use diesel_demo_steps_sqlite::core::connection::establish_connection;
use diesel_demo_steps_sqlite::core::post::*;

fn main() {
    use diesel_demo_steps_sqlite::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(connection)
        .unwrap();

    let post: Post = posts
        .find(id)
        .first(connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
