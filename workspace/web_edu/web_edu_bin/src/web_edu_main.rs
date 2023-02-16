
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
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use anyhow::Result;
use diesel::{RunQueryDsl, GroupedBy, QueryDsl, BelongingToDsl, TextExpressionMethods};
use web_edu_lib::core::connection::establish_connection_test;
use ::web_edu_lib::model::model::model_product::*;
use ::web_edu_lib::model::model::model_product_variant::*;
//use ::web_edu_lib::model::model::model_product_edit::{FormVariant, FormProductVariant, FormProductVariantComplete, FormProduct};
use web_edu_lib::schema::{products,products_variants, self};

/*fn main() {
    use schema::products::dsl::*;
    println!("The products are: {:#?}", index_list_products(products::table()));
    //println!("The products with variants are: {:#?}", list_products_variants());
}
 fn index_list_products( query: use schema::products::dsl::BoxedQuery<diesel::sqlite::Sqlite>) -> Vec<Product>
    where
           T: diesel::Table +
           diesel::query_builder::AsQuery,<T as diesel::query_builder::AsQuery>::Query: QueryId
  {
    
    let  mut conn = establish_connection_test();
     query    
    .limit(10)
    .load::<Product>(&mut conn)
    //.expect("Error loading products")
}*/
//  fn index_list_products<'a, C, T>(query: T)->Result<Vec<C>,Error>
//     where
//            T: diesel::Table +
//            diesel::query_builder::AsQuery,<T as diesel::query_builder::AsQuery>::Query: QueryId
//   {
    
//     let  mut conn = establish_connection_test();
//      query    
//     .limit(10)
//     .load::<T>(&mut conn)
//     //.expect("Error loading products")
// }

fn main() {
    
    println!("The products are: {:#?}", list_products());
    //let results = index_list_products();
    //println!("Displaying {} products", results.len());
    // for product in results {
    //     println!("{}", product.name);
    //     println!("----------\n");
    //     println!("{}", product.cost);
    // }
    //
    //println!("The products with variants are: {:#?}", list_products_variants());
}

fn list_products() -> Vec<Product> {
    use schema::products::dsl::*;
    let  conn = &mut establish_connection_test();
    products
    //.select(name)
    .limit(10)
    .load::<Product>(conn)
    .expect("Error loading products")
}