
#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// index
///
/// # Commands
/// > `Workspace`
/// ```RUST_BACKTRACE=1 cargo build -q -p diesel_demo_steps_sqlite```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
///
/// > `Sub Packges`
/// > > ` Binary `
/// ```cargo run -q -p diesel_demo_steps_sqlite --bin show_posts```
///
/// ```cargo doc  --package diesel_demo_steps_sqlite --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package diesel_demo_steps_sqlite```
///
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// `TODO`
/// 
use diesel::prelude::*;
use diesel_demo_steps_sqlite::models::*;
use diesel_demo_steps_sqlite::core::connection::establish_connection;

fn main() {
    use diesel_demo_steps_sqlite::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
