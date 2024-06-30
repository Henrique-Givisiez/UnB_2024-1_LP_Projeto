// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Int4,
        device_name -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    users,
);
