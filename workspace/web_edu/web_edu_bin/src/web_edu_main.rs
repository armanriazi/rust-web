#![crate_name = "web_edu_bin"]
#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// index
///
/// # Commands
/// > `Workspace`
/// ```cargo run -q -p web_edu_bin```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=json --all-targets```
///
/// ```cargo test --doc  --workspace```
///
/// > `Sub Packges`
/// > > ` Binary `
/// ```cargo run -q -p web_edu_bin --bin web_edu_index```
///
/// ```cargo doc  --package web_edu_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package web_edu_bin```
///
/// > > ` Library `
/// ```cargo run -q -p web_edu_lib```
///
/// ```cargo doc  --package web_edu_lib --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package web_edu_lib```
///
/// > > ` Model `
/// ```cargo run -q -p web_edu_model```
///
/// ```cargo doc  --package web_edu_model --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package web_edu_model```
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

use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;
use ::shared::connection::establish_connection;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel::{RunQueryDsl, GroupedBy, QueryDsl, BelongingToDsl, TextExpressionMethods};
use ::web_edu_model::models::{Product, NewCompleteProduct, NewProduct, NewVariantValue, NewVariant, ProductVariant, Variant};

fn main() {
    println!("The products are: {:#?}", index_list_products());
    //println!("The products with variants are: {:#?}", list_products_variants());
}

fn index_list_products() -> Vec<Product> {
    use ::web_edu::schema::products::dsl::*;
    let connection = establish_connection();
    products
        .limit(10)
        .load::<Product>(&connection)
        .expect("Error loading products")
}


fn list_products_variants(conn: &SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>, Error> {
    use ::web_edu::schema::products::dsl::products;
    use ::web_edu::schema::variants::dsl::variants;

    let products_result = 
        products
        .limit(10)
        .load::<Product>(conn)?;
    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}



fn show_product(id: i32, conn: &SqliteConnection) -> Result<Product, Error> {
    use ::web_edu::schema::products::dsl::products;

    products
        .find(id)
        .first(conn)
}


/// We use get_result instead of first because the return type has the trait BelongingToDsl implemented.
/// We use the belonging_to function in the ProductVariant struct because we use a foreign key, and Diesel automatically assigns the required trait for the association.
fn show_product_variant(id: i32, conn: &SqliteConnection) -> Result<(Product, Vec<(ProductVariant, Variant)>), Error> {
    use ::web_edu::schema::products::dsl::products;
    use ::web_edu::schema::variants::dsl::variants;

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



fn search_products(search: String, conn: &SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>, Error> {
    use ::shoe_store::schema::products::dsl::*;
    use ::shoe_store::schema::variants::dsl::variants;

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
use ::web_edu::connection::establish_connection_test;
use ::web_edu_model::models::{Product, NewCompleteProduct, NewProduct, NewVariantValue, NewVariant, ProductVariant, Variant};

fn index_list_products(conn: &SqliteConnection) -> Vec<Product> {
    use ::web_edu::schema::products::dsl::*;
    products
        .limit(10)
        .load::<Product>(conn)
        .expect("Error loading products")
}



#[test]
fn test_index_list_products() {
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        create_product(NewProduct {
            name: "boots".to_string(),
            cost: 13.23,
            active: true
        }, &connection);
        create_product(NewProduct {
            name: "high heels".to_string(),
            cost: 20.99,
            active: true
        }, &connection);
        create_product(NewProduct {
            name: "running shoes".to_string(),
            cost: 10.99,
            active: true
        }, &connection);

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
fn test_list_products_variants() {
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
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

        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            },
            variants: variants.clone()
        }, &connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "high heels".to_string(),
                cost: 20.99,
                active: true
            },
            variants: variants.clone()
        }, &connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "running shoes".to_string(),
                cost: 10.99,
                active: true
            },
            variants: variants.clone()
        }, &connection).unwrap();

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

        assert_eq!(serde_json::to_string(&list_products_variants(&connection).unwrap()).unwrap(), 
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
fn show_product_test() {

    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
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
            }, &connection).unwrap();

        assert_eq!(
            serde_json::to_string(&show_product(product_id, &connection).unwrap()).unwrap(),
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
fn show_product_test() {

    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
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
            }, &connection).unwrap();

        assert_eq!(
            serde_json::to_string(&show_product_variant(product_id, &connection).unwrap()).unwrap(),
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
}