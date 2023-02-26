
#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// index
///
/// # Commands
/// > `Workspace`
/// ```RUST_BACKTRACE=1 cargo run -q -p web_edu_bin```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
///
/// > `Sub Packges`
/// > > ` Binary `
/// ```cargo run -q -p web_edu_bin --bin web_edu_main```
///
/// ```cargo doc  --package web_edu_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package web_edu_bin```
///
/// > > ` Library `
/// ```cargo test -q -p web_edu_lib```
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

#[macro_use]
extern crate log;
extern crate diesel; // imported due to form edit include update, delete
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, ManageConnection};
use web_edu_lib::core::{connection::establish_connection, product_variant_create::product_variant::create_product_variant, product::product::list_products, product_variant::product_variant::list_products_variants};
use diesel::{RunQueryDsl, QueryDsl, SqliteConnection, Connection};
//use ::web_edu_lib::viewmodel::viewmodel::model_product_edit::{FormVariant, FormProductVariant, FormProductVariantComplete, FormProduct};
use web_edu_lib::schema::{*, self};
use web_edu_lib::core::connection::{database_url, database_test_url};
use actix_web::middleware::Logger;
use actix_web::{web::{self, Data}, middleware,  App, Error as AWError, HttpServer, Responder, HttpResponse,HttpRequest, Result, post, get};
use web_edu_lib::viewmodel::viewmodel::NewCompleteProduct;
use serde::{Deserialize, Serialize};
use r2d2_sqlite::{self, SqliteConnectionManager};
//
type DbError = Box<dyn std::error::Error + Send + Sync>;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
//
#[derive(Deserialize)]
struct QueryParamsList {
	limit: i64
}

// // This is like we can do a /products?limit=10
// // We need to pass the query parameters through a web::Query wrapper. 
// async fn product_list(pool: web::Data<DbPool>,query_param_list: web::Query<QueryParamsList>)
// 	-> Result<impl Responder> {
  //use schema::products::dsl::*;
//   let list = web::block(move ||  {
//     let mut conn = pool.get().expect("couldn't get db connection from pool");
//     match  actions::list_products(query_param_list.limit,  &mut conn)                
//      {
//       Ok(products) => Ok(web::Json(products)),
//       Err(error) => Err(actix_web::error::ErrorInternalServerError(error))
//     }
//   })?;
//    Ok(HttpResponse::Ok().json(list))
// }
// /// Our list endpoint, just make a call to list_product and pattern match
// /// the result, if everything is ok we're supposed to return a list of products
// /// as a json object with web::Json(products), otherise just return a 500 error.
// async fn products_variants_list(conn:web::Data<SqliteConnection>)
//     -> Result<impl Responder> {
   //use schema::products_variants::dsl::*;     
//     match list_products_variants(10 as i64,&mut conn) {
//         Ok(products) => Ok(web::Json(products)),
//         Err(error) => Err(actix_web::error::ErrorInternalServerError(error))
//     }
// }

// This endpoint will create a new product, we just call the 
// create_product function and expect a response, if everything is ok we just
// return a 200 response like this: HttpResponse::Ok(), otherwise
// we can return a 500 internal server error. 
async fn product_create(product: web::Json<NewCompleteProduct>) -> Result<impl Responder> {
	let connection = &mut establish_connection();
	match create_product_variant(&product, connection) {
		Ok(_) => Ok(HttpResponse::Ok()),
		Err(error) => Err(actix_web::error::ErrorInternalServerError(error))
	}
}


//----------------------------------------------//

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[derive(Serialize, Deserialize)]
struct Person {
	name: String,
	age: i32
}

async fn greet(req: HttpRequest) -> Result<impl Responder> {
    let name = req.match_info().get("name").unwrap_or("World");
   	if name == "Error" {
		Err(actix_web::error::ErrorInternalServerError("an error"))
	} else {
		Ok(web::Json(Person { name: "Peter Parker".to_string(), age: 32 }))
	}
}

//---------------------------//

#[derive(Debug)]
pub enum Error {
    ConnectionError(ConnectionError),
    QueryError(diesel::result::Error),
}

// In the main function we can create the routes to our endpoints.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));   
     
     let manager = SqliteConnectionManager::file(database_url().unwrap().as_str());
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(echo)
            .app_data(web::Data::new(pool.clone()))
            .route("/{name}", web::get().to(greet))
            .route("/hey", web::get().to(manual_hello))
            .route("products", web::post().to(product_create))
            //.route("products", web::get().to(product_list)) // our route to list products
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::web::Bytes; 
    use actix_web::{test, web, App};
    use diesel::Connection;
    use ::web_edu_lib::core::connection::establish_connection_test;
    use web_edu_lib::viewmodel::viewmodel::model_product::NewProduct;
    use web_edu_lib::viewmodel::viewmodel::model_variant::NewVariant;
    use ::web_edu_lib::viewmodel::viewmodel::{NewCompleteProduct, NewVariantValue};    

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .route("/{name}", web::get().to(greet)),
        )
        .await;
        let req = test::TestRequest::get().uri("/mundo").to_request();
        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, Bytes::from_static(b"welcome"));
    }

    #[actix_rt::test]
    async fn test_product_creation_is_ok() {
        let connection = &mut establish_connection_test();
        connection.begin_test_transaction().unwrap();

        // We can mock the server with init_service.
        let mut app = test::init_service(
            App::new()
              .data(connection)
              .route("products", web::post().to(product_create))
        ).await;

        let body =
          NewCompleteProduct {
            product: NewProduct {
              name: "boots".to_string(),
              cost: 13.23,
              active: true
            },
            variants: vec![
              NewVariantValue {
                variant: NewVariant {
                  name: "size".to_string()
                },
                values: vec![
                  Some(12.to_string()),
                  Some(14.to_string()),
                  Some(16.to_string()),
                  Some(18.to_string())
                ]
              }
            ]
          };

        // We test our endpoint by sending the body through a post request.
        let req = test::TestRequest::post().set_json(&body).uri("/products").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Expect a 200 response.
        assert!(resp.status().is_success());
    }
}