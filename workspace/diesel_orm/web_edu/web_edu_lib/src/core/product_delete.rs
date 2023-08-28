

#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
/// 
/// ```cargo test -q -p web_edu_lib delete_product_test  --  --show-output```
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
use crate::viewmodel::viewmodel::model_product::{NewProduct};
use crate::schema;
use diesel::sqlite::{SqliteConnection};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl};
use anyhow::Result;

pub fn delete_product(id: i32, conn: &mut SqliteConnection) ->  Result<i32> {
    use schema::products::dsl::products;
    diesel::delete(products.find(id))
        .execute(conn)?;

    Ok(id)
}


}

#[warn(unused_must_use)]
#[cfg(test)]
mod tests {
use diesel::connection::SimpleConnection;
use diesel::result::Error;
use diesel::{Connection, RunQueryDsl};
use crate::core::connection::establish_connection_test;
use crate::core::product::product::{ show_product};
use crate::core::product_delete::product::delete_product;
use crate::core::product_variant_create::product_variant::*;
use crate::model::ProductVariant;
use crate::viewmodel::viewmodel::{NewVariantValue, NewCompleteProduct};
use crate::viewmodel::viewmodel::model_product::{NewProduct};
use crate::viewmodel::viewmodel::model_variant::{NewVariant};
use crate::schema;

/// We need to be aware that SQLite does not enforce foreign keys, so we need to tell the database with the following command: connection.execute("PRAGMA foreign_keys = ON").
#[test]
#[should_panic(expected = "NotFound")]
fn delete_product_test() {
    use schema::products_variants::dsl::*;
    let connection = &mut establish_connection_test();
    connection.batch_execute("PRAGMA foreign_keys = ON").unwrap();
    connection.test_transaction::<_, Error, _>(|connection| {
     
        let created_product_id =
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
            }, 
            connection).unwrap();
        
        delete_product(created_product_id, connection).unwrap();

        let vec_product_variants = products_variants.load::<ProductVariant>(connection)?;
        assert_eq!(vec_product_variants, vec![]);
        show_product(created_product_id.try_into().unwrap(), connection).unwrap();

        Ok(())
    });
}
}