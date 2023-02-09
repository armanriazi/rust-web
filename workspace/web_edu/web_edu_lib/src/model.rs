pub mod model {

    // use self::{model_variant::NewVariant, model_product::NewProduct};
    
    // /// We might need additional models that have a different purpose and that aren’t connected to a table for our business logic.
    // #[derive(Clone)]
    // pub struct NewVariantValue {
    //     pub variant: NewVariant,
    //     pub values: Vec<Option<String>>
    // }

    // pub struct NewCompleteProduct {
    //     pub product: NewProduct,
    //     pub variants: Vec<NewVariantValue>
    // }

    // ----------------------------database models-------------

pub mod model_product {
        use diesel::Insertable;
        use crate::schema::*;
        //
        use serde::{Serialize, Deserialize};
        use diesel::Queryable;


        #[derive(Insertable, Debug)]
        #[table_name="products"]
        pub struct NewProduct {
            pub name: String,
            pub cost: f64,
            pub active: bool,
        }
        /// This struct will be our model for inserting data in our database. 
        /// Therefore, we need it to be Insertable, We also need to give it the name of our table.
        /// We’re now prepared to add the code corresponding to creating a products table.
        #[derive(Queryable, Debug, Serialize, Deserialize)]
        pub struct Product {
            pub id: i32,
            pub name: String,
            pub cost: f64,
            pub active: bool,
        }
}

// pub mod model_variant {
//         use diesel::{ Insertable, Identifiable, Queryable};
//         use serde::{Serialize, Deserialize};
//         use crate::schema::variants;

//         #[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
//         #[table_name = "variants"]
//         pub struct Variant {
//             pub id: i32,
//             pub name: String,
//         }

//         #[derive(Insertable, Debug, Clone)]
//         #[table_name="variants"]
//         pub struct NewVariant {
//             pub name: String,
//         }
//     }
//     pub mod model_product_variant {
//         use crate::schema::products_variants;

//         #[derive(Insertable, Debug)]
//         #[table_name="products_variants"]
//         pub struct NewProductVariant {
//             pub product_id: i32,
//             pub variant_id: i32,
//             pub value: Option<String>
//         }
//     }

//     pub mod model_product_edit {
//         use super::model_product::NewProduct;

//         #[derive(Insertable, Queryable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
//         #[table_name="variants"]
//         pub struct FormVariant {
//             pub id: Option<i32>,
//             pub name: String
//         }

//         #[derive(Insertable, Debug, AsChangeset)]
//         #[table_name="products_variants"]
//         pub struct FormProductVariant {
//             pub id: Option<i32>,
//             pub variant_id: Option<i32>,
//             pub product_id: i32,
//             pub value: Option<String>
//         }

//         pub struct FormProductVariantComplete {
//             pub variant: Option<FormVariant>,
//             pub product_variant: FormProductVariant,
//         }

//         pub struct FormProduct {
//             pub product: NewProduct,
//             pub variants: Vec<FormProductVariantComplete>
//         }
//     }

}


