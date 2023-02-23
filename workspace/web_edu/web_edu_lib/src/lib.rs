

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
/// ```cargo test -q -p web_edu_lib -- --show-output```
///
/// ```cargo doc  --package web_edu_lib --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package web_edu_lib```
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
#[macro_use]
extern crate diesel;
//#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

use diesel::prelude::*;
pub mod core;
pub mod model;
pub mod viewmodel;
pub mod schema;
