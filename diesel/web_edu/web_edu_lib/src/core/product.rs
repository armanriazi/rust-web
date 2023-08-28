

#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
/// 
/// ```cargo test -q -p web_edu_lib create_product_test  --  --show-output```
///
/// ```cargo test -q -p web_edu_lib list_products_test  --  --show-output```
/// 
/// ```cargo test -q -p web_edu_lib show_product_test  --  --show-output```
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
use crate::model::Product;
use diesel::{RunQueryDsl, QueryDsl};
use crate::schema;
use diesel::sqlite::{SqliteConnection};
use diesel::result::Error;


pub fn list_products(limit : i64, conn:  &mut SqliteConnection) -> Result<Vec<Product>, Error> {
    use schema::products::dsl::*;
    products
        .limit(limit)
        .load::<Product>(conn)
       // .expect("Error loading products")
}
pub fn show_product(id: i32, conn: &mut SqliteConnection) -> Result<Product, Error> {
    use schema::products::dsl::*;
    products
        .find(id)
        .first(conn)
}




}

#[warn(unused_must_use)]
#[cfg(test)]
mod tests {
use diesel::result::Error;
use diesel::{Connection};
use crate::core::connection::establish_connection_test;
use crate::core::product::product::{list_products, show_product};
use crate::core::product_create::product::create_product;
use crate::model::Product;
use crate::viewmodel::viewmodel::{NewVariantValue, NewCompleteProduct};
use crate::viewmodel::viewmodel::model_product::{NewProduct};
use crate::viewmodel::viewmodel::model_variant::{NewVariant};



#[test]
fn list_products_test() {
    let connection = &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        let a= create_product(&NewProduct {
            name: "boots".to_string(),
            cost: 13.23,
            active: true
        }, connection);
               
        let _= create_product(&NewProduct {
            name: "high heels".to_string(),
            cost: 20.99,
            active: true
        }, connection);
        let _= create_product(&NewProduct {
            name: "running shoes".to_string(),
            cost: 10.99,
            active: true
        }, connection);

        assert_eq!(serde_json::to_string(&list_products(10 as i64,connection).unwrap()).unwrap(), 
            serde_json::to_string(&vec![
                Product {
                    id: 1,
                    name: "boots".to_string(),
                    cost: 13.23,
                    active: true
                },
                Product {
                    id: 2,
                    name: "high heels".to_string(),
                    cost: 20.99,
                    active: true
                },
                Product {
                    id: 3,
                    name: "running shoes".to_string(),
                    cost: 10.99,
                    active: true
                }
            ]).unwrap());

        Ok(())

    });
}

#[test]
fn show_product_test() {
    use crate::core::product_variant_create::product_variant::*;
    
    let  connection = &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        
        let product_id =
        create_product_variant(&NewCompleteProduct {
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
        }, connection).unwrap();

        // The function list_products is created to list the last products with their variants.
        assert_eq!(
            serde_json::to_string(&show_product(product_id,connection).unwrap()).unwrap(),
            serde_json::to_string(                
                    &Product {
                        id: 1,
                        name: "boots".to_string(),
                        cost: 13.23,
                        active: true
                }
            ).unwrap()
        );

        Ok(())
    });
  }


}