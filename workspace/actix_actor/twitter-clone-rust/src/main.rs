
#![allow(dead_code, unused_variables)]
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

/// > `twitter-clone-rust`
///
/// # Commands
/// 
/// ```RUST_BACKTRACE=1 cargo run -q -p twitter-clone-rust```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
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


mod constants;
mod like;
mod response;
mod schema;
mod tweet;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tweet::list)
            .service(tweet::get)
            .service(tweet::create)
            .service(tweet::delete)
            .service(like::list)
            .service(like::plus_one)
            .service(like::minus_one)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
