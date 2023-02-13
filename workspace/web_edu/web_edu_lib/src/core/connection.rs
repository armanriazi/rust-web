
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("SQLITE_WEB_EDU_DATABASE_URL")
        .expect("SQLITE_WEB_EDU_DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_test() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("TEST_SQLITE_WEB_EDU_DATABASE_URL")
        .expect("TEST_SQLITE_WEB_EDU_DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}