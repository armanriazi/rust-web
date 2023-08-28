use diesel::prelude::*;
use super::schema::posts;
use super::schema::users;
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Queryable)]//, PartialEq, Debug
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub created_at:  NaiveDateTime,
    pub updated_at:  NaiveDateTime,
}
