
#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// index
///
/// # Commands
/// > `Workspace`
/// ```RUST_BACKTRACE=1 cargo run -q -p diesel_demo_step_2_sqlite```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
///
/// > `Sub Packges`
/// > > ` Binary `
/// ```cargo run -q -p diesel_demo_step_2_sqlite --bin write_post```
///
/// ```cargo doc  --package diesel_demo_step_2_sqlite --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package diesel_demo_step_2_sqlite```
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
use diesel_demo_step_2_sqlite::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    //dbg!(&title);
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        &title, EOF
    );
    let mut body = String::from("Body");
    //dbg!(&body);
    stdin().read_to_string(&mut body).unwrap();

    let ret = create_post(connection, title, &body);
    println!("\nSaved draft {} - ({})", title,ret);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
