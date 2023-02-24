

#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
/// 
/// ```cargo test -q -p web_edu_lib  create_product_variant  --  --show-output```
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


pub mod product_variant{
    use diesel::prelude::*;   
    use crate::model::ProductVariant;
    use crate::viewmodel::viewmodel::NewCompleteProduct;
    use crate::model::Product;
    use crate::model::Variant;       
    use crate::schema::{self};
    use diesel::sqlite::SqliteConnection;
    use diesel::result::Error;
    use diesel::RunQueryDsl;
    use diesel::BelongingToDsl;
    use anyhow::Result;    
    use diesel::ExpressionMethods;

    use diesel::Connection;    
    use diesel::query_dsl::QueryDsl;
    
   sql_function!{
    fn last_insert_rowid()-> diesel::sql_types::Integer;
   }

pub fn create_product_variant(new_product: &NewCompleteProduct, conn: &mut SqliteConnection) -> Result<i32>  {
        //use schema::products::dsl::*;
        use schema::variants::dsl::*;
        use schema::products_variants::dsl::*;

        conn.transaction(|conn| {
            diesel::insert_into(schema::products::dsl::products)
                .values(&new_product.product)
                .execute(conn)?;
            
            let last_product_id: i32 =  diesel::select(last_insert_rowid().into_sql::<diesel::sql_types::Integer>()).first(conn)?;                                    

            // In the code, we make a loop over the new variants and filter them by name to check if the variant was already created. 
            // We do this to avoid duplications and create it if necessary.
            for new_variant in &new_product.variants {
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

                            diesel::select(last_insert_rowid().into_sql::<diesel::sql_types::Integer>()).first(conn)?
                        }
                    };

                // Finally, we create the relationships needed for the products and variants.
                for new_value in &new_variant.values {
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