use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("SQLITE_DIESEL_DATABASE_URL").expect("SQLITE_DIESEL_DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[cfg(test)]
pub fn establish_connection() -> SqliteConnection {
    let url = ::std::env::var("SQLITE_DIESEL_DATABASE_URL").unwrap();
    SqliteConnection::establish(&url).unwrap()
}
