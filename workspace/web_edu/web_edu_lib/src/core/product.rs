

pub mod product{
use crate::model::model::model_product::{NewProduct, Product};
use crate::schema;
use diesel::sqlite::{SqliteConnection, SqliteType};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl};

pub fn create_product(new_product: &NewProduct, conn: &mut SqliteConnection) -> Result<usize, Error>  {
    use schema::products::dsl::*;
    diesel::insert_into(products)
        .values(new_product)
        .execute(conn)
        //.expect("Error saving new product")
}
pub fn list_products(conn:  &mut SqliteConnection) -> Vec<Product> {
    use schema::products::dsl::*;
    products
        .limit(10)
        .load::<Product>(conn)
        .expect("Error loading products")
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
use diesel::prelude::*;   
use diesel::result::Error;
use diesel::{Connection, SqliteConnection};
use crate::core::connection::establish_connection_test;
use crate::core::product::product::{create_product, list_products, show_product};
use crate::model::model::{NewVariantValue, NewCompleteProduct};
use crate::model::model::model_product::{NewProduct, Product};
use crate::model::model::model_variant::{Variant, NewVariant};
use crate::model::model::model_product_variant::NewProductVariant;
use crate::schema;

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


#[test]
fn list_products_test() {
    let connection = &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        let a= create_product(&NewProduct {
            name: "boots".to_string(),
            cost: 13.23,
            active: true
        }, connection);
        dbg!(a);
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

        assert_eq!(serde_json::to_string(&list_products(connection)).unwrap(), 
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
    let  connection = &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        
        let product_id =
        crate::core::product_variant::product_variant::create_product_variant(&NewCompleteProduct {
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