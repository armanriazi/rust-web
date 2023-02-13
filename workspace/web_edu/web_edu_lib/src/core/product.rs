

pub mod product{
use crate::model::model::model_product::{NewProduct, Product};
use crate::schema;
use diesel::sqlite::{SqliteConnection, SqliteType};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl};

pub fn create_product(new_product: NewProduct, conn: &SqliteConnection) -> Result<usize, Error>  {
    use schema::products::dsl::*;
    diesel::insert_into(products)
        .values(new_product)
        .execute(conn)
}
// pub fn list_products(conn:  SqliteConnection) -> Vec<Product> {
//     use schema::products::dsl::*;
//     products
//         .limit(10)
//         .load::<Product>(&conn)
//         .expect("Error loading products")
// }

}

#[warn(unused_must_use)]
#[cfg(test)]
mod tests {
use diesel::result::Error;
use diesel::{Connection, SqliteConnection};
use crate::core::connection::establish_connection_test;
use crate::core::product::product::{create_product};
use crate::model::model::model_product::{NewProduct};
use crate::model::model::model_variant::Variant;
use crate::model::model::model_product_variant::NewProductVariant;
use crate::schema;

#[test]
fn create_product_test() {
    let   connection = establish_connection_test();
    let _ =  &connection.test_transaction::<_, Error, _>(|| {
        let results = 
            create_product(NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            }, &connection);
        assert_eq!(Ok(1), results);

        Ok(())
    });
}

}