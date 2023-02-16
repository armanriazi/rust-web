
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

no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);// imported due to form edit include update, delete

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
    
    println!("The products are: {:#?}", index_list_products());
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

fn index_list_products() -> Vec<Product> {
    use schema::products::dsl::*;
    let  conn = &mut establish_connection_test();
    products
    //.select(name)
    .limit(10)
    .load::<Product>(conn)
    .expect("Error loading products")
}

/* 




fn show_product(id: i32, conn: &SqliteConnection) -> Result<Product, Error> {
    use ::web_edu_lib::schema::products::dsl::products;

    products
        .find(id)
        .first(conn)
}


/// We use get_result instead of first because the return type has the trait BelongingToDsl implemented.
/// We use the belonging_to function in the ProductVariant struct because we use a foreign key, and Diesel automatically assigns the required trait for the association.
fn show_product_variant(id: i32, conn: &mut SqliteConnection) -> Result<(Product, Vec<(ProductVariant, Variant)>), Error> {
    use ::web_edu_lib::schema::products::dsl::products;
    use ::web_edu_lib::schema::variants::dsl::variants;

    let product_result =
        products
            .find(id)
            .get_result::<Product>(conn)?;

    let variants_result =
        ProductVariant::belonging_to(&product_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?;

    Ok((product_result, variants_result))
}



fn search_products(search: String, conn: &mut SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>, Error> {
    use ::web_edu_lib::schema::products::dsl::*;
    use ::web_edu_lib::schema::variants::dsl::variants;

    let pattern = format!("%{}%", search);
    let products_result = 
        products
        .filter(name.like(pattern))
        .load::<Product>(conn)?;
    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}

/// We start updating the product, then loop through the variants and check if the field variant_id from product_variant is none. If so, it means we have a new variant, 
/// so we create and get the id. We add mut in the loop because we need to change the object later.
///
///We check if product_variant has an id so we can find out if we need to update a found record or create a new one.


fn update_product(product_id: i32, form_product: FormProduct, conn: &mut SqliteConnection) -> Result<i32> {
    use ::web_edu_lib::schema::products::dsl::products;
    use ::web_edu_lib::schema::variants;
    use ::web_edu_lib::schema::products_variants::dsl::products_variants;
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
                        diesel::select(last_insert_rowid).first(conn)?;

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

fn delete_product(id: i32, conn: &SqliteConnection) ->  Result<i32> {    
    use ::web_edu::schema::products::dsl::products;
    diesel::delete(products.find(id))
        .execute(conn)?;

    Ok(id)
}
 */
//-------------------------------tests------------------
// impl<T: Display> Display for Complex<T> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{} + {}i", self., self.)
//     }
// }
#[cfg(test)]
mod tests {
use super::*;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;
use diesel::result::Error;
use diesel::Connection;
use ::web_edu_lib::core::connection::establish_connection_test;
use web_edu_lib::core::product::product::create_product;
use ::web_edu_lib::model::model::model_product::{Product, NewProduct};
//use ::web_edu_lib::model::model::model_product::{Product, NewCompleteProduct, NewProduct, NewVariantValue, NewVariant, ProductVariant, Variant};
// use ::web_edu::schema::products_variants::dsl::*;
// use ::web_edu::schema::products::dsl::*;
// use ::web_edu::schema::variants;
//

pub fn index_list_products(conn: & SqliteConnection) -> Vec<Product> {
    use web_edu_lib::schema::products::dsl::*;
    products
        .limit(10)
        .load::<Product>(conn)
        .expect("Error loading products")
}



#[test]
fn test_index_list_products() {
    use ::web_edu_lib::schema::products::dsl::*;

    let connection = &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        create_product(&NewProduct {
            name: "boots".to_string(),
            cost: 13.23,
            active: true
        }, connection);
        create_product(&NewProduct {
            name: "high heels".to_string(),
            cost: 20.99,
            active: true
        }, connection);
        create_product(&NewProduct {
            name: "running shoes".to_string(),
            cost: 10.99,
            active: true
        }, connection);

        assert_eq!(serde_json::to_string(&index_list_products(&connection)).unwrap(), 
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

//

#[test]
fn show_product_test() {

    let mut connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        let product_id =
            create_product(NewCompleteProduct {
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
            serde_json::to_string(&show_product(product_id.try_into().unwrap(), &connection).unwrap()).unwrap(),
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
  
  //

#[test]
fn show_product_variant_test() {

    let mut connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        let product_id =
            create_product(NewCompleteProduct {
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
            }, &mut connection).unwrap();

        assert_eq!(
            serde_json::to_string(&show_product_variant(product_id.try_into().unwrap(), &mut connection).unwrap()).unwrap(),
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

  //

/* 
#[test]
fn search_products_test() {
    
    use ::web_edu_lib::schema::variants::dsl::variants;

    let mut connection = establish_connection_test();
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

        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "high heels".to_string(),
                cost: 20.99,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "running shoes".to_string(),
                cost: 10.99,
                active: true
            },
            variants: variants.clone()
        }, connection).unwrap();

        assert_eq!(
            serde_json::to_string(&search_products("shoes".to_string(), connection).unwrap()).unwrap(),
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

  //

#[test]
fn update_product_test() {
    use ::web_edu_lib::schema::products::dsl::products;
    use ::web_edu_lib::schema::variants;
    use ::web_edu_lib::schema::products_variants::dsl::products_variants;

    let mut connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        let created_product_id =
            create_product(NewCompleteProduct {
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
                .filter(product_id.eq(created_product_id))
                .load::<ProductVariant>(&connection)
                .unwrap();

        let product_variant =
            vec_product_variant
                .first()
                .unwrap();

        update_product(
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
            &connection
        ).unwrap();
        
        assert_eq!(
            serde_json::to_string(&show_product(created_product_id, &connection).unwrap()).unwrap(),
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

  //

#[test]
#[should_panic(expected = "NotFound")]
fn delete_product_test() {
    use ::web_edu::schema::products_variants::dsl::products_variants;
    let connection = establish_connection_test();
    connection.execute("PRAGMA foreign_keys = ON").unwrap();
    connection.test_transaction::<_, Error, _>(|| {
        let created_product_id =
            create_product(NewCompleteProduct {
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
            &connection).unwrap();
        
        delete_product(created_product_id, &connection).unwrap();

        let vec_product_variants = products_variants.load::<ProductVariant>(&connection)?;
        assert_eq!(vec_product_variants, vec![]);
        show_product(created_product_id, &connection).unwrap();

        Ok(())
    });
  }
*/
}