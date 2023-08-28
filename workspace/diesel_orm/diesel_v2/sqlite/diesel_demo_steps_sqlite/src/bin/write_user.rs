/// > ` Binary `
/// ```cargo run -q -p diesel_demo_steps_sqlite --bin write_user```
///
/// ```cargo doc  --package diesel_demo_steps_sqlite --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package diesel_demo_step_3_sqlite```
///
use std::io::{stdin, Read};
use diesel::prelude::*;
use diesel_demo_steps_sqlite::models::*;
use diesel_demo_steps_sqlite::core::connection::establish_connection;
use diesel_demo_steps_sqlite::core::user::*;

fn main() {
    let connection = &mut establish_connection();
    
    let x = insert_tuple_batch(connection);
    println!("\nSaved draft {:?}", x.unwrap());
}
