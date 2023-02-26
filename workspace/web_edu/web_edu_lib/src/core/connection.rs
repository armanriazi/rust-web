
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {


    SqliteConnection::establish(&database_url().unwrap())
        .expect(&format!("Error connecting to {}", &database_url().unwrap()))
}

pub fn establish_connection_test() -> SqliteConnection {

    SqliteConnection::establish(&database_test_url().unwrap())
        .expect(&format!("Error connecting to {}", &database_test_url().unwrap()))
}

pub fn database_url()-> Result<String,dotenv::Error>{
    dotenv().ok();
    Ok(env::var("SQLITE_WEB_EDU_DATABASE_URL").expect("SQLITE_WEB_EDU_DATABASE_URL must be set"))
}
pub fn database_test_url()-> Result<String,dotenv::Error>{
    dotenv().ok();
    Ok(env::var("TEST_SQLITE_WEB_EDU_DATABASE_URL").expect("TEST_SQLITE_WEB_EDU_DATABASE_URL must be set"))
}