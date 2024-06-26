// @generated automatically by Diesel CLI.

diesel::table! {
    devices (device_id) {
        device_id -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    users,
);
