#![allow(dead_code, unused_variables)]
//#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]
/// Unit Tests
///
/// # Commands
/// > `Test per unit`
///
/// ```cargo test -q -p web_edu_lib create_sale_test  --  --show-output```
///
/// ```cargo test -q -p web_edu_lib list_sale_test  --  --show-output```
///
/// ```cargo test -q -p web_edu_lib  show_sale_test  -- --show-output```
///
/// ```cargo test -q -p web_edu_lib  delete_sale_test  -- --show-output```
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
    use diesel::{IntoSql, RunQueryDsl, QueryDsl, Connection};
    use diesel::result::Error;
    use diesel::sqlite::SqliteConnection;
    use crate::model::Sale;

    sql_function!{
        fn now() -> diesel::sql_types::Date;
    }


    pub fn create_sale(sale_str: String, conn: &mut SqliteConnection) -> Result<usize, Error> {
        use schema::sales::dsl::*;

        let _sale: NewSale = serde_json::from_str(sale_str.as_str()).unwrap();

        //println!("{:?}",&_sale);
        let new_sales = NewSale::new(_sale.date.to_string(),_sale.tax_total.into(), _sale.sub_total.into());
        //println!("{:?}",&new_sales);
        diesel::insert_into(sales)
        .values(&new_sales)
        .execute(conn)        
        
    }

    pub fn show_sale(sale_id: i32, conn: &mut SqliteConnection) -> Result<Sale, Error> {
         use schema::sales::dsl::*;
            sales
            .find(&sale_id)
            .first(conn)
    }

    pub fn update_sale(sale_id: i32, form_sale: FormSale, conn: &mut SqliteConnection) -> Result<usize, Error> {
        use schema::sales::dsl::*;
        // We begin a transaction, just to make sure everything runs successfully
        conn.transaction(|conn| {
            diesel::update(sales.find(sale_id))
                .set(&form_sale)
                .execute(conn)?;        
            Ok(sale_id as usize)
        })      
    }

    pub fn delete_sale(sale_id: i32, conn:  &mut SqliteConnection) -> Result<usize, Error> {
        use schema::sales::dsl::*;
        diesel::delete(sales.find(sale_id))
        .execute(conn)?;

        Ok(sale_id.try_into().unwrap())
    }


}

#[warn(unused_must_use)]
#[cfg(test)]
mod tests {
use diesel::result::Error;
use diesel::{Connection, RunQueryDsl, Table};
use crate::core::connection::establish_connection_test;
use crate::core::sale::sale::{create_sale, show_sale, delete_sale, update_sale};
use crate::model::Sale;
use crate::schema;
use crate::viewmodel::viewmodel::model_sale_edit::FormSale;
use crate::viewmodel::viewmodel::model_sales::NewSale;
#[test]
fn create_sale_test() {
        let  connection = &mut establish_connection_test();
        let new_sale_str= r#"{
                    "date": "2022/02/02",
                    "tax_total": 5,
                    "sub_total": 5                    
                }
                "#;
        let _ =  &connection.test_transaction::<_, Error, _>(|connection| {
            let results = 
                    create_sale(new_sale_str.to_string(),            
                    connection);
            assert_eq!(Ok(1), results);
            Ok(())
        });
    }

#[test]
fn show_sale_test() {        
    use schema::sales::dsl::*;

    let connection =  &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {

                let new_sale_str= r#"{
                    "date": "2022/02/02",
                    "tax_total": 5,
                    "sub_total": 5                    
                }
                "#;
                let created_sale_id=create_sale(new_sale_str.to_string(),connection).unwrap();
                assert_eq!(
                serde_json::to_string(&show_sale(created_sale_id.try_into().unwrap(), connection).unwrap()).unwrap(),
                serde_json::to_string(                    
                      &Sale {
                            id: 1,
                            date: "2022/02/02".to_string(),
                            tax_total: 5,
                            sub_total: 5,
                            total: 10,
                        }                
            ).unwrap()
        );

        Ok(())
    });
  }

//

#[test]
#[should_panic(expected = "NotFound")]
fn delete_sale_test() {
    use schema::sales::dsl::*;
    let connection = &mut establish_connection_test();
   // connection.execute("PRAGMA foreign_keys = ON").unwrap();
    connection.test_transaction::<_, Error, _>(|connection| {
        let new_sale_str= r#"{
            "date": "2022/02/02",
            "tax_total": 5,
            "sub_total": 5                    
        }
        "#;
        let created_sale_id=create_sale(new_sale_str.to_string(),connection).unwrap();

        delete_sale(created_sale_id.try_into().unwrap(), connection).unwrap();

        let vec_sales = sales.load::<Sale>(connection)?;
        assert_eq!(vec_sales, vec![]);
        show_sale(created_sale_id.try_into().unwrap(), connection).unwrap();

        Ok(())
    });
}
//
#[test]
fn update_sale_test() {
    use schema::sales::dsl::*;
 use crate::diesel::ExpressionMethods;
    let connection =  &mut establish_connection_test();
    connection.test_transaction::<_, Error, _>(|connection| {
        let new_sale_str= r#"{
            "date": "2022/02/02",
            "tax_total": 5,
            "sub_total": 5                    
        }
        "#;
        let created_sale_id=create_sale(new_sale_str.to_string(),connection).unwrap();

        let vec_sale = diesel::QueryDsl::filter(sales, sales::all_columns().0.eq(*&created_sale_id as i32))
                .load::<Sale>(connection)
                .unwrap();
            
        let sale =
            vec_sale
                .first()
                .unwrap();

        let updated_sale_id = update_sale(
            created_sale_id as i32,
            FormSale {                             
                    date: "2022/03/03".to_string(),
                    tax_total: Some(5),
                    sub_total: Some(5),
                    total: Some(10),                
            },
            connection
        ).unwrap();
        
                //let created_sale_id=create_sale(new_sale_str.to_string(),connection).unwrap();
                assert_eq!(
                serde_json::to_string(&show_sale(updated_sale_id.try_into().unwrap(), connection).unwrap()).unwrap(),
                serde_json::to_string(                    
                      &Sale {
                            id: 1,
                            date: "2022/03/03".to_string(),
                            tax_total: 5,
                            sub_total: 5,
                            total: 10,
                        }                
            ).unwrap()
        );

        Ok(())
    });
}
}