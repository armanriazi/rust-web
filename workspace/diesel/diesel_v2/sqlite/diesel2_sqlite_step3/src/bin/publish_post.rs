/// > ` Binary `
/// ```cargo run -q -p diesel_demo_step_3_sqlite --bin publish_post your_id```
///
/// ```cargo doc  --package diesel_demo_step_3_sqlite --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package diesel_demo_step_3_sqlite```
///
use diesel::prelude::*;
use diesel_demo_step_3_sqlite::*;
use std::env::args;

fn main() {
    use diesel_demo_step_3_sqlite::schema::posts::dsl::{posts, published};

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

    let post: models::Post = posts
        .find(id)
        .first(connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
