#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
///
/// ```cargo test -q -p web_edu_lib create_sale_test  -- --exact  --show-output```
///
/// ```cargo test -q -p web_edu_lib list_sale_test  -- --exact  --show-output```
///
/// ```cargo test -q -p web_edu_lib show_sale_test  -- --exact  --show-output```
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

pub mod sale {
    use crate::viewmodel::viewmodel::model_sale_edit::FormSale;
    use crate::schema::{sales, self};
    use crate::viewmodel::viewmodel::model_sales::NewSale;
    use diesel::result::Error;
    use diesel::sqlite::SqliteConnection;
    use crate::model::Sale;

    pub fn create_sale(sale_str: String, conn: &mut SqliteConnection) -> Result<usize, Error> {
        use schema::sales::dsl::*;
        // let sales = vec![NewSale {
        //     date: "2023/02/01".to_string(),
        //     tax_total: 0,
        //     sub_total: 0,
        //     total: 0,
        // }];

        // diesel::insert_into(sales).values(sale_str).execute(conn);
        //.expect("Error saving new product")
        unimplemented!();
    }

    pub fn show_sale(sale_id: i32, conn: &mut SqliteConnection) -> Result<Sale, Error> {
         use schema::sales::dsl::*;
        // sales.find(sale_id).first(conn)
        unimplemented!();
    }

    pub fn update_sale(sale_id: i32, form_sale: FormSale, conn: &mut SqliteConnection) -> Result<usize, Error> {
        use schema::sales::dsl::*;
        unimplemented!();
    }

    pub fn delete_sale(sale_id: i32, conn:  &mut SqliteConnection) -> Result<usize, Error> {
        use schema::sales::dsl::*;
        unimplemented!();
    }
}
