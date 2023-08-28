

#![allow(dead_code, unused_variables)]



//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// index
///
/// # Commands
/// > `Workspace`
/// 
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
/// 
/// > > ` Library `
/// ```cargo test -q -p diesel_demo_steps_sqlite -- --show-output```
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


#[macro_use]
extern crate diesel;
//#[macro_use] extern crate diesel_codegen;
extern crate dotenvy;
extern crate serde;
extern crate serde_json;

use diesel::prelude::*;
pub mod core;
pub mod models;
pub mod viewmodels;
pub mod schema;
