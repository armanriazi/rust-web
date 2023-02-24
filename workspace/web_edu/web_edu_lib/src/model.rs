 
use serde::{Serialize, Deserialize};
use diesel::prelude::*;   
use diesel::{Identifiable,Queryable,Selectable};        
use diesel::query_dsl::BelongingToDsl;
use crate::schema::*;

#[derive(Identifiable,Queryable, Debug, Serialize, Deserialize,PartialEq,Selectable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}



#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize,PartialEq,Selectable)]       
pub struct Variant {
    pub id: i32,
    pub name: String,
}




#[derive(Identifiable,Associations, Queryable, Debug, Serialize, Deserialize,PartialEq,Eq,Selectable)]
#[diesel(table_name = products_variants)]      
#[diesel(belongs_to(Product, foreign_key = product_id))]
pub struct ProductVariant {
    pub id: i32,
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>
}


#[derive(Identifiable, Queryable, Clone, Debug, PartialEq, Serialize, Deserialize,Selectable,Eq)]
#[diesel(table_name = sales)]   
pub struct Sale {
    pub id: i32,
    pub date: String,
    pub tax_total: i32,
    pub sub_total: i32,
    pub total: i32
}