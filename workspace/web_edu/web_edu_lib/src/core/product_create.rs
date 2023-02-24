

#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
/// 
/// ```cargo test -q -p web_edu_lib create_product_test  --  --show-output```
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
/// 

pub mod product{
use crate::viewmodel::viewmodel::model_product::*;
use diesel::{RunQueryDsl};
use crate::schema;
use diesel::sqlite::{SqliteConnection};
use diesel::result::Error;

pub fn create_product(new_product: &NewProduct, conn: &mut SqliteConnection) -> Result<usize, Error>  {
    use schema::products::dsl::*;
    diesel::insert_into(products)
        .values(new_product)
        .execute(conn)
        //.expect("Error saving new product")
}
}

#[warn(unused_must_use)]
#[cfg(test)]
mod tests {
use diesel::result::Error;
use diesel::{Connection};
use crate::core::connection::establish_connection_test;
use crate::core::product_create::product::create_product;
use crate::viewmodel::viewmodel::model_product::{NewProduct};

#[test]
fn create_product_test() {
    let  connection = &mut establish_connection_test();
    let _ =  &connection.test_transaction::<_, Error, _>(|connection| {
        let results = 
            create_product(&NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            }, connection);
        assert_eq!(Ok(1), results);

        Ok(())
    });
}
}