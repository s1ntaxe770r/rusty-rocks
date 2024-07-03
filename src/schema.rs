// @generated automatically by Diesel CLI.

diesel::table! {
    rocks (id) {
        id -> Int4,
        name -> Varchar,
        kind -> Varchar,
    }
}
