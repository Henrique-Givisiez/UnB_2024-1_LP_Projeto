use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::{Device, User};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_all_devices(conn: &PgConnection) -> QueryResult<Vec<Device>> {
    use crate::schema::devices::dsl::*;
    devices.load::<Device>(conn)
}

pub fn get_device_by_id(conn: &PgConnection, device_id: &str) -> QueryResult<Device> {
    use crate::schema::devices::dsl::*;
    devices.filter(device_id.eq(device_id)).first::<Device>(conn)
}

pub fn create_device(conn: &PgConnection, new_device: &Device) -> QueryResult<usize> {
    use crate::schema::devices;
    diesel::insert_into(devices::table)
        .values(new_device)
        .execute(conn)
}

pub fn update_device(conn: &PgConnection, device_id: &str, new_status: &str) -> QueryResult<usize> {
    use crate::schema::devices::dsl::*;
    diesel::update(devices.filter(device_id.eq(device_id)))
        .set(status.eq(new_status))
        .execute(conn)
}

pub fn delete_device(conn: &PgConnection, device_id: &str) -> QueryResult<usize> {
    use crate::schema::devices::dsl::*;
    diesel::delete(devices.filter(device_id.eq(device_id)))
        .execute(conn)
}

pub fn create_user(conn: &PgConnection, new_user: &User) -> QueryResult<usize> {
    use crate::schema::users;
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
}

pub fn get_user_by_username(conn: &PgConnection, username: &str) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    users.filter(username.eq(username)).first::<User>(conn)
}
