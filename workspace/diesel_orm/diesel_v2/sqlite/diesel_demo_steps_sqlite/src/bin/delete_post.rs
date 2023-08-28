/// > ` Binary `
/// ```cargo run -q -p diesel_demo_steps_sqlite --bin delete_post your_titled```
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
    use diesel_demo_steps_sqlite::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
