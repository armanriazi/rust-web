

pub mod create_product{
use crate::model::model::model_product::NewProduct;
use crate::schema;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel::RunQueryDsl;

pub fn create_product(new_product: NewProduct, conn: &mut SqliteConnection) -> Result<usize, Error>  {
    use schema::products::dsl::*;
    diesel::insert_into(products)
        .values(new_product)
        .execute(conn)
}
}

#[cfg(test)]
mod tests {

use diesel::result::Error;
use diesel::Connection;
use crate::core::connection::establish_connection;
use crate::core::product::create_product::{create_product};
use crate::model::model::model_product::NewProduct;

#[test]
fn create_product_test() {
    let mut connection = establish_connection();
    let _ = &connection.test_transaction::<_, Error, _>(|connection| {
        let results = 
            create_product(NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            },connection);
        assert_eq!(Ok(1), results);

        Ok(())
    });
}
}