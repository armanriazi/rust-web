/// > ` Binary `
/// ```cargo run -q -p diesel_demo_step_3_sqlite --bin write_post```
///
/// ```cargo doc  --package diesel_demo_step_3_sqlite --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package diesel_demo_step_3_sqlite```
///
use diesel_demo_step_3_sqlite::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let _ = create_post(connection, title, &body);
    println!("\nSaved draft {}", title);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
