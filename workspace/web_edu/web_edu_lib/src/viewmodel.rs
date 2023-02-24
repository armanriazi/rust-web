/// ```cargo test -q -p web_edu_lib```
///
pub mod viewmodel {

    use serde::{Deserialize, Serialize};

    use self::model_product::NewProduct;
    use self::model_variant::NewVariant;

    /// We might need additional models that have a different purpose and that aren’t connected to a table for our business logic.
    #[derive(Clone, Serialize, Deserialize)]
    pub struct NewVariantValue {
        pub variant: NewVariant,
        pub values: Vec<Option<String>>,
    }
    #[derive(Clone, Serialize, Deserialize)]
    pub struct NewCompleteProduct {
        pub product: NewProduct,
        pub variants: Vec<NewVariantValue>,
    }

    // ----------------------------database models-------------

    pub mod model_product {
        use crate::schema::*;
        use diesel::{Insertable, Queryable,AsChangeset};
        use serde::{Deserialize, Serialize};

        /// This struct will be our model for inserting data in our database.
        /// Therefore, we need it to be Insertable, We also need to give it the name of our table.

        #[derive(Insertable, Debug, AsChangeset, Serialize, Deserialize, Clone)]
        //#[table_name="products"] //v1
        #[diesel(table_name = products)]
        pub struct NewProduct {
            pub name: String,
            pub cost: f64,
            pub active: bool,
        }
        
    }

    pub mod model_variant {
        use crate::schema::*;
        use diesel::{Identifiable, Insertable, Queryable, Selectable};
        use serde::{Deserialize, Serialize};

        #[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, PartialEq, Selectable)]
        pub struct Variant {
            pub id: i32,
            pub name: String,
        }

        #[derive(Serialize, Deserialize)]
        #[derive(Insertable, Debug, Clone)]
        #[diesel(table_name = variants)]
        pub struct NewVariant {
            pub name: String,
        }
    }

    pub mod model_product_variant {
        use crate::schema::products_variants;
        use diesel::prelude::*;
        use diesel::query_dsl::BelongingToDsl;
        use diesel::{ Insertable};

        #[derive(Insertable, Debug)]
        #[diesel(table_name = products_variants)]
        pub struct NewProductVariant {
            pub product_id: i32,
            pub variant_id: i32,
            pub value: Option<String>,
        }
    }

    pub mod model_product_edit {
        use super::model_product::NewProduct;
        use crate::schema::products_variants;
        use crate::schema::variants;
        use serde::{Deserialize, Serialize};
        use diesel::{ Insertable, AsChangeset};
        
        #[derive(Insertable, Queryable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
        #[diesel(table_name = variants)]
        pub struct FormVariant {
            pub id: Option<i32>,
            pub name: String,
        }

        #[derive(Insertable, Debug, AsChangeset)]
        #[diesel(table_name = products_variants)]
        pub struct FormProductVariant {
            pub id: Option<i32>,
            pub variant_id: Option<i32>,
            pub product_id: i32,
            pub value: Option<String>,
        }

        pub struct FormProductVariantComplete {
            pub variant: Option<FormVariant>,
            pub product_variant: FormProductVariant,
        }

        pub struct FormProduct {
            pub product: NewProduct,
            pub variants: Vec<FormProductVariantComplete>,
        }
    }

    pub mod model_sales {

        use crate::schema::sales::{self, sub_total};
        use diesel::{Insertable,AsChangeset};
        use serde::{Deserialize, Serialize};
        
        #[derive(Insertable, Debug, AsChangeset, Serialize, Deserialize)]
        #[diesel(table_name = sales)]
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
    }


    pub mod model_sale_edit{        
        use serde::{Deserialize, Serialize};
        use crate::schema::sales;
        use diesel::{Insertable,AsChangeset};

        #[derive(Insertable, Debug, AsChangeset, Serialize, Deserialize)]
        #[diesel(table_name = sales)]
        pub struct FormSale {
            pub date: String,
            pub tax_total: Option<i32>,
            pub sub_total: Option<i32>,
            pub total: Option<i32>
        }
    }
}
