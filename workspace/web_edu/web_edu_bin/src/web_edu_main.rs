
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
extern crate diesel; // imported due to form edit include update, delete
use web_edu_lib::core::connection::establish_connection;
use diesel::{RunQueryDsl, QueryDsl};
use web_edu_lib::core::connection::establish_connection_test;
use ::web_edu_lib::model::*;
use ::web_edu_lib::viewmodel::viewmodel::model_product_edit::{FormVariant, FormProductVariant, FormProductVariantComplete, FormProduct};
use web_edu_lib::schema::{self};

use actix_web::{web, App, HttpServer, Responder, HttpResponse, Result};

use ::web_edu_lib::models::NewCompleteProduct;


// This endpoint will create a new product, we just call the 
// create_product function and expect a response, if everything is ok we just
// return a 200 response like this: HttpResponse::Ok(), otherwise
// we can return a 500 internal server error. 
async fn product_create(product: web::Json<NewCompleteProduct>) -> Result<impl Responder> {
	let connection = establish_connection();
	match create_product(product.clone(), &connection) {
		Ok(_) => Ok(HttpResponse::Ok()),
		Err(error) => Err(actix_web::error::ErrorInternalServerError(error))
	}
}

// In the main function we can create the routes to our endpoints.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("products", web::post().to(product_create))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}