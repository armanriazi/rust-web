```rust

#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate diesel;
use shoe_store::schema::sales;
use serde::{Serialize, Deserialize};
use diesel::{RunQueryDsl, QueryDsl,Insertable,AsChangeset};


#[derive(Identifiable, Queryable, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[table_name="sales"]
pub struct Sale {
    pub id: i32,
    pub date: String,
    pub tax_total: i32,
    pub sub_total: i32,
    pub total: i32
}
        
#[derive(Insertable, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name="sales"]
pub struct NewSale {
    pub date: String,
    pub tax_total: i32,
    pub sub_total: i32,     
    #[serde(skip)]    
    total: i32
}

impl  NewSale{
    pub fn new(_date:String,_tax_total:i32,_sub_total:i32) -> NewSale{

    NewSale {
    date: _date,
    tax_total: _tax_total,
    sub_total: _sub_total,
    total: _tax_total + _sub_total
    }            
}

}

pub fn create_sale(sale_str: String, conn: &SqliteConnection) -> Result<usize, Error> {
    use ::shoe_store::schema::sales::dsl::*;    

    let _sale: NewSale = serde_json::from_str(sale_str.as_str()).unwrap();

    //println!("{:?}",&_sale);
    let new_sales = NewSale::new(_sale.date.to_string(),_sale.tax_total.into(), _sale.sub_total.into());
    //println!("{:?}",&new_sales);
    diesel::insert_into(sales)
    .values(&new_sales)
    .execute(conn)        
    
}

pub fn show_sale(sale_id: i32, conn: &SqliteConnection) -> Result<Sale, Error> {
        use ::shoe_store::schema::sales::dsl::*;
        sales
        .find(sale_id)
        .first(conn)
}



#[test]
fn create_sale_test() {
    use ::shoe_store::schema::sales::dsl::*; 

    let  connection =establish_connection_test();
    let new_sale_str= r#"{
                "date": "2022/02/02",
                "tax_total": 5,
                "sub_total": 5                    
            }
            "#;
    let _ =  &connection.test_transaction::<_, Error, _>(|| {
        let results = 
                create_sale(new_sale_str.to_string(),            
                &connection);
        assert_eq!(Ok(1), results);
        Ok(())
    });
}

#[test]
fn show_sale_test() {    
use ::shoe_store::schema::sales::dsl::*;    
let connection =  establish_connection_test();
connection.test_transaction::<_, Error, _>(|| {

            let new_sale_str= r#"{
                "date": "2022/02/02",
                "tax_total": 5,
                "sub_total": 5                    
            }
            "#;
            let created_sale_id=create_sale(new_sale_str.to_string(),&connection).unwrap();
            assert_eq!(
            serde_json::to_string(&show_sale(created_sale_id.try_into().unwrap(), &connection).unwrap()).unwrap(),
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

#![allow(dead_code, unused_variables)]
#[macro_use]
extern crate diesel;
use shoe_store::schema::sales;
use serde::{Serialize, Deserialize};


#[derive(Insertable, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name="sales"]
pub struct FormSale {
    pub date: String,
    pub tax_total: Option<i32>,
    pub sub_total: Option<i32>,
    pub total: Option<i32>
}

fn update_sale(sale_id: i32, form_sale: FormSale, conn: &SqliteConnection) -> Result<usize, Error> {
        use ::shoe_store::schema::sales::dsl::*; 
    
            diesel::update(sales.find(sale_id))
                .set(&form_sale)
                .execute(conn)?;        
            Ok(sale_id as usize)
    
}

fn delete_sale(sale_id: i32, conn:  &SqliteConnection) -> Result<usize, Error> {
        use ::shoe_store::schema::sales::dsl::*; 
        diesel::delete(sales.find(sale_id))
        .execute(conn)?;

        Ok(sale_id as usize)
}


#[test]
#[should_panic(expected = "NotFound")]
fn delete_sale_test() {
     use ::shoe_store::schema::sales::dsl::*; 
    let connection =  establish_connection_test();
   // connection.execute("PRAGMA foreign_keys = ON").unwrap();
    connection.test_transaction::<_, Error, _>(|| {
        let new_sale_str= r#"{
            "date": "2022/02/02",
            "tax_total": 5,
            "sub_total": 5                    
        }
        "#;
        let created_sale_id=create_sale(new_sale_str.to_string(),&connection).unwrap();

        delete_sale(created_sale_id.try_into().unwrap(), &connection).unwrap();

        let vec_sales = sales.load::<Sale>(&connection)?;
        assert_eq!(vec_sales, vec![]);
        show_sale(created_sale_id as i32, &connection).unwrap();

        Ok(())
    });
}


#[test]
fn update_sale_test() {
     use ::shoe_store::schema::sales::dsl::*; 
     use crate::diesel::ExpressionMethods;
     let connection =  establish_connection_test();
     &connection.test_transaction::<_, Error, _>(|| {
        let new_sale_str= r#"{
            "date": "2022/02/02",
            "tax_total": 5,
            "sub_total": 5                    
        }
        "#;
        let created_sale_id=create_sale(new_sale_str.to_string(),&connection).unwrap();

        let vec_sale = diesel::QueryDsl::filter(sales, sales::all_columns().0.eq(*&created_sale_id as i32))
                .load::<Sale>(&connection)
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
            &connection
        ).unwrap();
        
                //let created_sale_id=create_sale(new_sale_str.to_string(),connection).unwrap();
                assert_eq!(
                serde_json::to_string(&show_sale(updated_sale_id.try_into().unwrap(), &connection).unwrap()).unwrap(),
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
```

> Output:

```
"{\"id\":1,\"date\":\"01/01/2020\",\"tax_total\":210000,\"sub_total\":78456895,\"total\":78666895}"
"{\"id\":2,\"date\":\"02/01/2020\",\"tax_total\":250000,\"sub_total\":56741125,\"total\":56991125}"
"{\"id\":3,\"date\":\"03/01/2020\",\"tax_total\":240000,\"sub_total\":10000000,\"total\":10240000}"
"{\"id\":4,\"date\":\"04/01/2020\",\"tax_total\":260000,\"sub_total\":96452470,\"total\":96712470}"
"{\"id\":5,\"date\":\"05/01/2020\",\"tax_total\":310000,\"sub_total\":86632254,\"total\":86942254}"
```