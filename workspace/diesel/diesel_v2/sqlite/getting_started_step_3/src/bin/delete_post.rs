/// > ` Binary `
/// ```cargo run -q -p diesel_demo_step_3_sqlite --bin delete_post your_titled```
///
/// ```cargo doc  --package diesel_demo_step_3_sqlite --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package diesel_demo_step_3_sqlite```
///
use diesel::prelude::*;
use diesel_demo_step_3_sqlite::*;
use std::env::args;

fn main() {
    use diesel_demo_step_3_sqlite::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
