

pub mod create_product{
    use ::web_edu::models::NewProduct;
    use diesel::sqlite::SqliteConnection;
    use diesel::result::Error;
    use diesel::RunQueryDsl;
    //use shared::core::connection::establish_connection;


    /// The insert_into function needs a target. In this case, the target is products, which comes from the DSL module from the products schema. We can use the values method to insert the data. It can be a single value or several. For our use case, we need a single product that will be used in the future endpoint.
    /// This function will return the number of rows inserted if it is successful. Otherwise, it will return an error.
    fn create_product(new_product: NewProduct, conn: &SqliteConnection) -> Result<usize, Error>  {
        use ::web_edu::schema::products::dsl::*;
        diesel::insert_into(products)
            .values(new_product)
            .execute(conn)
    }
}