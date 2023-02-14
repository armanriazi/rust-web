
#[macro_use]
extern crate diesel;

extern crate dotenv;
extern crate serde;
extern crate serde_json;

use diesel::prelude::*;
pub mod core;
pub mod model;
pub mod schema;
