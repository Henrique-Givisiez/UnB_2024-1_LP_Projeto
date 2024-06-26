table! {
    devices (device_id) {
        device_id -> Varchar,
        status -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}
