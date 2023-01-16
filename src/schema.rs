// @generated automatically by Diesel CLI.

diesel::table! {
    heroes (id) {
        id -> Int4,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Int4,
    }
}
