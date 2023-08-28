use diesel::prelude::*;
use super::schema::posts;
use super::schema::users;
use chrono::NaiveDateTime;
use serde::Deserialize;


#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


#[derive(Deserialize,Insertable)]
#[diesel(table_name = users)]
pub struct UserForm<'a> {
    pub name: &'a str,
    pub hair_color: &'a str,
}
