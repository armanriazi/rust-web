
#![allow(dead_code, unused_variables)]
use diesel::prelude::*;
use diesel_demo_steps_sqlite::models::*;
use diesel_demo_steps_sqlite::core::connection::establish_connection;
use diesel_demo_steps_sqlite::schema;
use chrono::NaiveDateTime;
#[cfg(test)]
use diesel::prelude::*;
use diesel::debug_query;
use diesel::insert_into;
#[cfg(test)]
use diesel::sqlite::Sqlite;
use serde::Deserialize;
use std::error::Error;
use diesel::QueryResult;
use diesel::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::query_dsl::methods::SelectDsl;
use diesel::Connection;
use diesel::ExpressionMethods;


/// index
///
/// # Commands
/// > `Workspace`
/// ```RUST_BACKTRACE=1 cargo build -q -p diesel_demo_steps_sqlite```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
///
/// > `Sub Packges`
/// > > ` Binary `
/// ```cargo test -q -p diesel_demo_steps_sqlite --bin tests_user```
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
/// 

/*
fn main() {
    use diesel_demo_steps_sqlite::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}*/

fn main(){
    unimplemented!();

#[test]
fn examine_sql_from_insert_default_values() {
    use schema::users::dsl::*;

    let query = insert_into(users).default_values();
    let sql = "INSERT INTO `users` DEFAULT VALUES -- binds: []";
    assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}

#[test]
fn examine_sql_from_insert_single_column() {
    use schema::users::dsl::*;

    let query = insert_into(users).values(name.eq("Sean"));
    let sql = "INSERT INTO `users` (`name`) VALUES (?) \
               -- binds: [\"Sean\"]";
    assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}

#[test]
fn examine_sql_from_insert_multiple_columns() {
    use schema::users::dsl::*;

    let query = insert_into(users).values((name.eq("Tess"), hair_color.eq("Brown")));
    let sql = "INSERT INTO `users` (`name`, `hair_color`) VALUES (?, ?) \
               -- binds: [\"Tess\", \"Brown\"]";
    assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}


#[test]
fn examine_sql_from_insertable_struct() {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Sean", "hair_color": "Black" }"#;
    let user_form = serde_json::from_str::<UserForm>(json).unwrap();
    let query = insert_into(users).values(&user_form);
    let sql = "INSERT INTO `users` (`name`, `hair_color`) VALUES (?, ?) \
               -- binds: [\"Sean\", \"Black\"]";
    assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}

#[test]
fn examine_sql_from_insertable_struct_option() {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Ruby", "hair_color": null }"#;
    let user_form = serde_json::from_str::<UserForm>(json).unwrap();
    let query = insert_into(users).values(&user_form);
    let sql = "INSERT INTO `users` (`name`) VALUES (?) \
               -- binds: [\"Ruby\"]";
    assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}
#[test]
fn examine_sql_from_insert_single_column_batch() {
    // Sorry, we can't inspect this as a single query on SQLite.
    // You can loop over the values to see each individual insert statement.
    //
    // use schema::users::dsl::*;

    // let values = vec![name.eq("Sean"), name.eq("Tess")];
    // let query = insert_into(users).values(&values);
    // let sql = "INSERT INTO `users` (`name`) VALUES (?), (?) \
    //            -- binds: [\"Sean\", \"Tess\"]";
    // assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}


#[test]
fn examine_sql_from_insert_single_column_batch_with_default() {
    // Sorry, we can't inspect this as a single query on SQLite.
    // You can loop over the values to see each individual insert statement.
    //
    // use schema::users::dsl::*;

    // let values = vec![Some(name.eq("Sean")), None];
    // let query = insert_into(users).values(&values);
    // let sql = "INSERT INTO `users` (`name`) VALUES (?), (DEFAULT) \
    //            -- binds: [\"Sean\"]";
    // assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}



#[test]
fn examine_sql_from_insert_tuple_batch() {
    // Sorry, we can't inspect this as a single query on SQLite.
    // You can loop over the values to see each individual insert statement.
    //
    // use schema::users::dsl::*;

    // let values = vec![
    //     (name.eq("Sean"), hair_color.eq("Black")),
    //     (name.eq("Tess"), hair_color.eq("Brown")),
    // ];
    // let query = insert_into(users).values(&values);
    // let sql = "INSERT INTO `users` (`name`, `hair_color`) \
    //            VALUES (?, ?), (?, ?) \
    //            -- binds: [\"Sean\", \"Black\", \"Tess\", \"Brown\"]";
    // assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}

#[test]
fn examine_sql_from_insert_tuple_batch_with_default() {
    // Sorry, we can't inspect this as a single query on SQLite.
    // You can loop over the values to see each individual insert statement.
    //
    // use schema::users::dsl::*;

    // let values = vec![
    //     (name.eq("Sean"), Some(hair_color.eq("Black"))),
    //     (name.eq("Ruby"), None),
    // ];
    // let query = insert_into(users).values(&values);
    // let sql = "INSERT INTO `users` (`name`, `hair_color`) \
    //            VALUES (?, ?), (?, DEFAULT) \
    //            -- binds: [\"Sean\", \"Black\", \"Ruby\"]";
    // assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}
#[test]
fn examine_sql_from_insertable_struct_batch() {
    // Sorry, we can't inspect this as a single query on SQLite.
    // You can loop over the values to see each individual insert statement.
    //
    // use schema::users::dsl::*;

    // let json = r#"[
    //     { "name": "Sean", "hair_color": "Black" },
    //     { "name": "Tess", "hair_color": "Brown" }
    // ]"#;
    // let user_form = serde_json::from_str::<Vec<UserForm>>(json).unwrap();
    // let query = insert_into(users).values(&user_form);
    // let sql = "INSERT INTO `users` (`name`, `hair_color`) \
    //            VALUES (?, ?), (?, ?) \
    //            -- binds: [\"Sean\", \"Black\", \"Tess\", \"Brown\"]";
    // assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}

#[test]
fn insert_get_results_batch() {
    use diesel::result::Error;
    use diesel::select;
    use schema::users::dsl::*;
    let conn = &mut establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {


        let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;

        let inserted_users = conn.transaction::<_, Error, _>(|conn| {
            let inserted_count = insert_into(users)
                .values(&vec![
                    (id.eq(1), name.eq("Sean")),
                    (id.eq(2), name.eq("Tess")),
                ])
                .execute(conn)?;

            Ok(users
                .order(id.desc())
                .limit(inserted_count as i64)
                .load(conn)?
                .into_iter()
                .rev()
                .collect::<Vec<_>>())
        })?;

        let expected_users = vec![
            User {
                id: 1,
                name: "Sean".into(),
                hair_color: None,
                created_at: now,
                updated_at: now,
            },
            User {
                id: 2,
                name: "Tess".into(),
                hair_color: None,
                created_at: now,
                updated_at: now,
            },
        ];
        assert_eq!(expected_users, inserted_users);

        Ok(())
    });
}

#[test]
fn examine_sql_from_insert_get_results_batch() {
    use schema::users::dsl::*;

    // Sorry, we can't inspect this as a single query on SQLite.
    // You can loop over the values to see each individual insert statement.
    //
    // let values = vec![(id.eq(1), name.eq("Sean")), (id.eq(2), name.eq("Tess"))];
    // let insert_query = insert_into(users).values(&values);
    // let insert_sql = "INSERT INTO `users` (`id`, `name`) VALUES (?, ?), (?, ?) \
    //            -- binds: [1, \"Sean\", 2, \"Tess\"]";
    // assert_eq!(insert_sql, debug_query::<Sqlite, _>(&insert_query).to_string());
    let load_query = users.order(id.desc());
    let load_sql = "SELECT `users`.`id`, `users`.`name`, \
                    `users`.`hair_color`, `users`.`created_at`, \
                    `users`.`updated_at` \
                    FROM `users` \
                    ORDER BY `users`.`id` DESC \
                    -- binds: []";
    assert_eq!(load_sql, debug_query::<Sqlite, _>(&load_query).to_string());
}

// FIXME: This test fails with "database is locked" for no obvious reason
// #[test]
// fn insert_get_result() {
//     use diesel::result::Error;

//     let conn = &mut establish_connection();
//     conn.test_transaction::<_, Error, _>(|conn| {
//         use diesel::select;
//         use schema::users::dsl::*;

//         let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(&conn)?;

//         let inserted_user = conn.transaction::<_, Error, _>(|conn| {
//             insert_into(users)
//                 .values((id.eq(3), name.eq("Ruby")))
//                 .execute(&conn)?;

//             users.order(id.desc()).first(&conn)
//         })?;

//         let expected_user = User {
//             id: 3,
//             name: "Ruby".into(),
//             hair_color: None,
//             created_at: now,
//             updated_at: now,
//         };
//         assert_eq!(expected_user, inserted_user);

//         Ok(())
//     });
// }

#[test]
fn examine_sql_from_insert_get_result() {
    use schema::users::dsl::*;

    let insert_query = insert_into(users).values((id.eq(3), name.eq("Ruby")));
    let insert_sql = "INSERT INTO `users` (`id`, `name`) VALUES (?, ?) -- binds: [3, \"Ruby\"]";
    assert_eq!(
        insert_sql,
        debug_query::<Sqlite, _>(&insert_query).to_string()
    );
    let load_query = users.order(id.desc());
    let load_sql = "SELECT `users`.`id`, `users`.`name`, \
                    `users`.`hair_color`, `users`.`created_at`, \
                    `users`.`updated_at` \
                    FROM `users` \
                    ORDER BY `users`.`id` DESC \
                    -- binds: []";
    assert_eq!(load_sql, debug_query::<Sqlite, _>(&load_query).to_string());
}


#[test]
fn examine_sql_from_explicit_returning() {
    use schema::users::dsl::*;

    let insert_query = insert_into(users).values(name.eq("Ruby"));
    let insert_sql = "INSERT INTO `users` (`name`) VALUES (?) -- binds: [\"Ruby\"]";
    assert_eq!(
        insert_sql,
        debug_query::<Sqlite, _>(&insert_query).to_string()
    );
    let load_query = QueryDsl::select(users, id).order(id.desc());//users.select(id).order(id.desc());
    let load_sql = "SELECT `users`.`id` FROM `users` ORDER BY `users`.`id` DESC -- binds: []";
    assert_eq!(load_sql, debug_query::<Sqlite, _>(&load_query).to_string());
}