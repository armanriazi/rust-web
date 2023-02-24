

#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
/// 
/// ```cargo test -q -p web_edu_lib update_product_variant_test  --  --show-output```
///
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


pub mod product_variant_edit {

use diesel::{SqliteConnection, Connection, QueryDsl, RunQueryDsl, IntoSql};
use anyhow::Result;
use crate::{schema, viewmodel::viewmodel::model_product_edit::FormProduct};

sql_function!{
fn last_insert_rowid()-> diesel::sql_types::Integer;
}

/// We start updating the product, then loop through the variants and check if the field variant_id from product_variant is none. If so, it means we have a new variant, so we create and get the id. We add mut in the loop because we need to change the object later.
/// We check if product_variant has an id so we can find out if we need to update a found record or create a new one.

pub fn update_product_variant(product_id: i32, form_product: FormProduct, conn: &mut SqliteConnection) -> Result<i32> {
    use schema::products::dsl::products;
    use schema::variants;
    use schema::products_variants::dsl::products_variants;
   
    // We begin a transaction, just to make sure everything runs successfully
    conn.transaction(|conn| {
        diesel::update(products.find(product_id))
            .set(&form_product.product)
            .execute(conn)?;

        // We just loop through all variants available
        for mut form_product_variant in form_product.variants {
            // If there's no variant id, we probably need to insert a new one.
            if form_product_variant.product_variant.variant_id.is_none() {
                diesel::insert_into(variants::dsl::variants)
                    .values(form_product_variant.variant)
                    .execute(conn)?;

                let last_variant_id: i32 =
                        diesel::select(last_insert_rowid().into_sql::<diesel::sql_types::Integer>()).first(conn)?; 

                form_product_variant.product_variant.variant_id = Some(last_variant_id);            
            }
            // We have an id, so we should update the variant product.
            if let Some(product_variant_id) = form_product_variant.product_variant.id {
                diesel::update(products_variants.find(product_variant_id))
                    .set(&form_product_variant.product_variant)
                    .execute(conn)?;
            }
        }

        Ok(product_id)
    })
}


#[cfg(test)]
mod tests {
use crate::core::connection::establish_connection_test;
use crate::core::product::product::show_product;
use crate::core::product_variant_create::product_variant::create_product_variant;
use crate::core::product_variant_edit::product_variant_edit::update_product_variant;
use crate::model::{ProductVariant, Product};
use crate::viewmodel::viewmodel::model_product_edit::{FormProductVariantComplete, FormProductVariant, FormVariant};
use crate::viewmodel::viewmodel::{NewCompleteProduct, NewVariantValue};
use diesel::result::Error;
use diesel::{Connection, Table, RunQueryDsl};
use crate::core::product_variant_create::*;
use crate::viewmodel::viewmodel::model_product::{NewProduct};
use crate::viewmodel::viewmodel::model_variant::{NewVariant, Variant};
use crate::{schema, viewmodel::viewmodel::model_product_edit::FormProduct};
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;



/// In the test above, we create a new product, then modify a variant with the new value 50. Next, we add a new variant, color, with the value Blue.
/// Then, we use show_product to compare it and check if the change was successful.
#[test]
fn update_product_variant_test() {
    use schema::products_variants::dsl::*;

    let connection =  &mut establish_connection_test();
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
            }, connection).unwrap();

        let vec_product_variant =
            products_variants
                .filter(products_variants::all_columns().2.eq(&created_product_id))
                .load::<ProductVariant>(connection)
                .unwrap();

        let product_variant =
            vec_product_variant
                .first()
                .unwrap();

        update_product_variant(
            created_product_id,
            FormProduct {
                product: NewProduct {
                    name: "high heels".to_string(),
                    cost: 14.25,
                    active: false
                },
                variants: vec![
                    FormProductVariantComplete {
                        variant: Some(FormVariant {
                            id: None,
                            name: "color".to_string()
                        }),
                        product_variant: FormProductVariant {
                            id: None,
                            product_id: created_product_id,
                            variant_id: None,
                            value: Some("Blue".to_string())
                        }
                    },
                    FormProductVariantComplete {
                        variant: None,
                        product_variant: FormProductVariant {
                            id: Some(product_variant.id),
                            product_id: created_product_id,
                            variant_id: Some(product_variant.variant_id),
                            value: Some(50.to_string())
                        }

                    }
                ]
            },
            connection
        ).unwrap();
        
        assert_eq!(
            serde_json::to_string(&show_product(created_product_id, connection).unwrap()).unwrap(),
            serde_json::to_string(
                &(
                    Product {
                        id: 1,
                        name: "high heels".to_string(),
                        cost: 14.25,
                        active: false
                    },
                    vec![
                        (
                            ProductVariant {
                                id: 1,
                                variant_id: 1,
                                product_id: 1,
                                value: Some(
                                    "50".to_string(),
                                ),
                            },
                            Variant {
                                id: 1,
                                name: "size".to_string(),
                            }
                        ),
                        (
                            ProductVariant {
                                id: 2,
                                variant_id: 1,
                                product_id: 1,
                                value: Some(
                                    "14".to_string(),
                                ),
                            },
                            Variant {
                                id: 1,
                                name: "size".to_string(),
                            }
                        ),
                        (
                            ProductVariant {
                                id: 3,
                                variant_id: 1,
                                product_id: 1,
                                value: Some(
                                    "16".to_string(),
                                ),
                            },
                            Variant {
                                id: 1,
                                name: "size".to_string(),
                            }
                        ),
                        (
                            ProductVariant {
                                id: 4,
                                variant_id: 1,
                                product_id: 1,
                                value: Some(
                                    "18".to_string(),
                                ),
                            },
                            Variant {
                                id: 1,
                                name: "size".to_string(),
                            }
                        ),
                        (
                            ProductVariant {
                                id: 5,
                                variant_id: 2,
                                product_id: 1,
                                value: Some(
                                    "Blue".to_string(),
                                ),
                            },
                            Variant {
                                id: 2,
                                name: "color".to_string(),
                            }
                        )
                    ]
                )
            ).unwrap()
        );

        Ok(())
    });
}
}
}