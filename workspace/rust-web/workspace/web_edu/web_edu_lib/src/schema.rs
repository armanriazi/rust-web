// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Nullable<Integer>,
        name -> Text,
        cost -> Double,
        active -> Bool,
    }
}
