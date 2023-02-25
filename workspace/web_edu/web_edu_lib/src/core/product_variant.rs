

#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
/// 
/// ```cargo test -q -p web_edu_lib  show_product_variants_test  --  --show-output```
///
/// ```cargo test -q -p web_edu_lib  show_products_variants_test  --  --show-output```
///
/// ```cargo test -q -p web_edu_lib  search_products_test  --  --show-output```
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
///
/// we use the macro no_arg_sql_function!, which allows us to use an SQL function in our code. 
/// In this case, SQLite does not have RETURNING for our inserts implemented.
/// Therefore, we need a function called last_insert_rowid to get the last id inserted.
// Note: We must be careful to use this function in a transaction.
/// 1.
///```rust,no_run,compile_fail
///no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);
///```
/// 2.
/// ```rust,no_run,compile_fail
///  sql_function!{
///     /// Represents the SQL NOW() function
///     fn now() -> sql_types::Timestamp;
/// }
/// ```
/// 3.
/// `SELECT id from table ORDER BY id DESC LIMIT 1;`
/// 
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


pub mod product_variant{
    use diesel::prelude::*;   
    use crate::model::ProductVariant;
    use crate::model::Product;
    use crate::model::Variant;       
    use crate::schema::{self};
    use diesel::sqlite::SqliteConnection;
    use diesel::result::Error;
    use diesel::RunQueryDsl;
    use diesel::BelongingToDsl;
    use anyhow::Result;    
    use diesel::ExpressionMethods;
    use diesel::query_dsl::QueryDsl;
    
   sql_function!{
    fn last_insert_rowid()-> diesel::sql_types::Integer;
   }

pub fn list_products_variants(limit : i64, conn: &mut SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>, Error> {
    use schema::products::dsl::*;
    use schema::variants::dsl::*;

    let products_result=  products
        //.select(name)
        .limit(limit)
        .load::<Product>(conn)?;
       

    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}

pub fn search_products_variants(search: String, conn: &mut SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>, Error> {
    use schema::products::dsl::*;
    use schema::variants::dsl::*;

    let pattern = format!("%{}%", search);
    let products_result = 
        products
        .filter(products::all_columns().1.like(pattern))
        .load::<Product>(conn)?;
    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}

}


// impl<T: Display> Display for Complex<T> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{} + {}i", self., self.)
//     }
// }
#[cfg(test)]
mod tests {
use crate::core::connection::establish_connection_test;
use crate::core::product::product::show_product;
use crate::viewmodel::viewmodel::{NewCompleteProduct, NewVariantValue};
use diesel::result::Error;
use diesel::Connection;
use crate::core::product_variant::product_variant::{ list_products_variants, search_products_variants};
use crate::core::product_variant_create::product_variant::*;
use crate::viewmodel::viewmodel::model_product::{NewProduct};
use crate::viewmodel::viewmodel::model_variant::{NewVariant, Variant};
use crate::model::*;

    
   sql_function!{
    fn last_insert_rowid()-> diesel::sql_types::Integer;
   }

#[test]
fn show_products_variants_test() {

    let connection = &mut establish_connection_test();

    connection.test_transaction::<_, Error, _>(|connection| {
        let variants = vec![
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
        ];

        create_product_variant(&NewCompleteProduct {
            product: NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();
        create_product_variant(&NewCompleteProduct {
            product: NewProduct {
                name: "high heels".to_string(),
                cost: 20.99,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();
        create_product_variant(&NewCompleteProduct {
            product: NewProduct {
                name: "running shoes".to_string(),
                cost: 10.99,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();

        let variants_result = |start_id, for_product_id| {
            vec![
                (
                    ProductVariant {
                        id: start_id + 1,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some(
                            "12".to_string(),
                        ),
                    },
                    Variant {
                        id: 1,
                        name: "size".to_string(),
                    }
                ),
                (
                    ProductVariant {
                        id: start_id + 2,
                        variant_id: 1,
                        product_id: for_product_id,
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
                        id: start_id + 3,
                        variant_id: 1,
                        product_id: for_product_id,
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
                        id: start_id + 4,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some(
                            "18".to_string(),
                        ),
                    },
                    Variant {
                        id: 1,
                        name: "size".to_string(),
                    }
                )
            ]
        };

        assert_eq!(serde_json::to_string(&list_products_variants(10 as i64,connection).unwrap()).unwrap(), 
            serde_json::to_string(&vec![
                (
                    Product {
                        id: 1,
                        name: "boots".to_string(),
                        cost: 13.23,
                        active: true
                    },
                    variants_result(0, 1)
                ),
                (
                    Product {
                        id: 2,
                        name: "high heels".to_string(),
                        cost: 20.99,
                        active: true
                    },
                    variants_result(4, 2)
                ),
                (
                    Product {
                        id: 3,
                        name: "running shoes".to_string(),
                        cost: 10.99,
                        active: true
                    },
                    variants_result(8, 3)
                )
            ]).unwrap());

        Ok(())

    });
}


#[test]
fn show_product_variants_test() {

    let connection = &mut establish_connection_test();
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

        assert_eq!(
            serde_json::to_string(&show_product(product_id, connection).unwrap()).unwrap(),
            serde_json::to_string(
                &(
                    Product {
                        id: 1,
                        name: "boots".to_string(),
                        cost: 13.23,
                        active: true
                    },
                    vec![
                        (
                            ProductVariant {
                                id: 1,
                                variant_id: 1,
                                product_id: 1,
                                value: Some(
                                    "12".to_string(),
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
                        )
                    ]
                )
            ).unwrap()
        );

        Ok(())
    });
}


#[test]
fn search_products_test() {
    let connection = &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        let variants = vec![
            NewVariantValue {
                variant: NewVariant {
                    name: "size".to_string()
                },
                values: vec![
                    Some(12.to_string()),
                ]
            }
        ];

        create_product_variant(&NewCompleteProduct {
            product: NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();
        create_product_variant(&NewCompleteProduct {
            product: NewProduct {
                name: "high heels".to_string(),
                cost: 20.99,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();
        create_product_variant(&NewCompleteProduct {
            product: NewProduct {
                name: "running shoes".to_string(),
                cost: 10.99,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();

        assert_eq!(
            serde_json::to_string(&search_products_variants("shoes".to_string(), connection).unwrap()).unwrap(),
            serde_json::to_string(&vec![
                (
                    Product {
                        id: 3,
                        name: "running shoes".to_string(),
                        cost: 10.99,
                        active: true
                    },
                    vec![
                        (
                            ProductVariant {
                                id: 3,
                                variant_id: 1,
                                product_id: 3,
                                value: Some(
                                    "12".to_string(),
                                ),
                            },
                            Variant {
                                id: 1,
                                name: "size".to_string(),
                            }
                        )
                    ]
                )
            ]).unwrap()
        );

        Ok(())
    });
}
}