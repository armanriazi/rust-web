use chrono::NaiveDateTime;
#[cfg(test)]
use diesel::prelude::*;
use diesel::debug_query;
use diesel::insert_into;
#[cfg(test)]
use diesel::sqlite::Sqlite;
use serde::Deserialize;
use std::error::Error;
use crate::schema;
use diesel::QueryResult;
use diesel::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::query_dsl::methods::SelectDsl;
use diesel::Connection;
use crate::viewmodels::UserForm;

pub fn insert_default_values(conn: &mut SqliteConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users).default_values().execute(conn)
}



pub fn insert_single_column(conn: &mut SqliteConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users).values(name.eq("Sean")).execute(conn)
}


pub fn insert_multiple_columns(conn: &mut SqliteConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values((name.eq("Tess"), hair_color.eq("Brown")))
        .execute(conn)
}

pub fn insert_insertable_struct(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Sean", "hair_color": "Black" }"#;
    let user_form = serde_json::from_str::<UserForm>(json)?;

    insert_into(users).values(&user_form).execute(conn)?;

    Ok(())
}


pub fn insert_insertable_struct_option(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Ruby", "hair_color": null }"#;
    let user_form = serde_json::from_str::<UserForm>(json)?;

    insert_into(users).values(&user_form).execute(conn)?;

    Ok(())
}



pub fn insert_single_column_batch(conn: &mut SqliteConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![name.eq("Sean"), name.eq("Tess")])
        .execute(conn)
}


pub fn insert_single_column_batch_with_default(conn: &mut SqliteConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![Some(name.eq("Sean")), None])
        .execute(conn)
}


pub fn insert_tuple_batch(conn: &mut SqliteConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![
            (name.eq("Sean"), hair_color.eq("Black")),
            (name.eq("Tess"), hair_color.eq("Brown")),
        ])
        .execute(conn)
}


pub fn insert_tuple_batch_with_default(conn: &mut SqliteConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![
            (name.eq("Sean"), Some(hair_color.eq("Black"))),
            (name.eq("Ruby"), None),
        ])
        .execute(conn)
}



pub fn insert_insertable_struct_batch(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;

    let json = r#"[
        { "name": "Sean", "hair_color": "Black" },
        { "name": "Tess", "hair_color": "Brown" }
    ]"#;
    let user_form = serde_json::from_str::<Vec<UserForm>>(json)?;

    insert_into(users).values(&user_form).execute(conn)?;

    Ok(())
}

/*
pub fn explicit_returning(conn: &mut SqliteConnection) -> QueryResult<i32> {
    use diesel::result::Error;
    use schema::users::dsl::*;

    conn.transaction::<_, Error, _>(|conn| {
        insert_into(users).values(name.eq("Ruby")).execute(conn)?;
        
         QueryDsl::select(users, id).order(id.desc()).first(conn)//users.select(id).order(id.desc()).first(conn)
    })
}
*/

