
pub mod create_product{

    #[macro_use]
    extern crate diesel;

    use anyhow::Result;
    use diesel::sqlite::SqliteConnection;
    use diesel::ExpressionMethods;
    use ::web_edu_model_product::models::*;//web_edu_model_variant
    use diesel::Connection;
    use diesel::RunQueryDsl;
    use diesel::query_dsl::QueryDsl;

    /// we use the macro no_arg_sql_function!, which allows us to use an SQL function in our code. 
    /// In this case, SQLite does not have RETURNING for our inserts implemented.
    /// Therefore, we need a function called last_insert_rowid to get the last id inserted.
    // Note: We must be careful to use this function in a transaction.
    no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

    fn create_product(new_product: NewCompleteProduct, conn: &SqliteConnection) -> Result<i32>  {
        use ::web_edu::schema::products::dsl::products;
        use ::web_edu::schema::variants::dsl::*;
        use ::web_edu::schema::products_variants::dsl::*;

        conn.transaction(|| {
            diesel::insert_into(products)
                .values(new_product.product)
                .execute(conn)?;

            let last_product_id: i32 = diesel::select(last_insert_rowid).first(conn)?;

            // In the code, we make a loop over the new variants and filter them by name to check if the variant was already created. 
            // We do this to avoid duplications and create it if necessary.
            for new_variant in new_product.variants {
                let variants_result =
                    variants
                        .filter(name.eq(&new_variant.variant.name))
                        .limit(1)
                        .load::<Variant>(conn)?;

                let last_variant_id: i32 =
                    match variants_result.first() {
                        Some(variant) => variant.id,
                        None => {
                            diesel::insert_into(variants)
                                .values(name.eq(&new_variant.variant.name))
                                .execute(conn)?;

                            diesel::select(last_insert_rowid).first(conn)?
                        }
                    };

                // Finally, we create the relationships needed for the products and variants.
                for new_value in new_variant.values {
                    diesel::insert_into(products_variants)
                        .values(
                            (
                                product_id.eq(last_product_id), 
                                variant_id.eq(last_variant_id),
                                value.eq(new_value), 
                            )
                        )
                        .execute(conn)?;
                }
            }
            Ok(last_product_id)
        })
    }
    
}


// impl<T: Display> Display for Complex<T> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{} + {}i", self., self.)
//     }
// }
#[cfg(test)]
mod tests {
use super::*;
use ::shared::connection::establish_connection_test;
use diesel::result::Error;

#[test]
fn create_product_test() {
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
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

        // The function list_products is created to list the last products with their variants.
        assert_eq!(
            serde_json::to_string(&list_products(&connection).unwrap()).unwrap(),
            serde_json::to_string(&vec![
                (
                    Product {
                        id: 1,
                        name: "boots".to_string(),
                        cost: 13.23,
                        active: true
                    },
                    vec![
                        (
                            Some(12.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(14.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(16.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(18.to_string()),
                            "size".to_string()
                        )
                    ]
                ),
            ]).unwrap()
        );

        Ok(())
    });
  }
}