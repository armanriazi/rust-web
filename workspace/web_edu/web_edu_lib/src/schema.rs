// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Nullable<Integer>,
        name -> Text,
        cost -> Double,
        active -> Bool,
    }
}

diesel::table! {
    products_variants (id) {
        id -> Integer,
        variant_id -> Integer,
        product_id -> Integer,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    variants (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    sales (id) {
        id -> Integer,
        date -> Text,
        tax_total -> Text,
        sub_total -> Text,
        total -> Text,
    }
}

diesel::joinable!(products_variants -> products (product_id));
diesel::joinable!(products_variants -> variants (variant_id));

diesel::allow_tables_to_appear_in_same_query!(
    products,
    products_variants,
    variants,
);
