use crate::database::{get_device_by_id, update_device, create_device, get_user_by_username, create_user};
use crate::models::{Device, DeviceData, User};
use diesel::PgConnection;

pub fn handle_update_device(conn: &PgConnection, data: DeviceData) -> Result<(), String> {
    match get_device_by_id(conn, &data.device_id) {
        Ok(_) => {
            update_device(conn, &data.device_id, &data.status).map_err(|e| e.to_string())?;
            Ok(())
        },
        Err(_) => {
            let new_device = Device {
                device_id: data.device_id,
                status: data.status,
            };
            create_device(conn, &new_device).map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}

pub fn handle_login_user(conn: &PgConnection, username: &str, password: &str) -> Result<User, String> {
    let user = get_user_by_username(conn, username).map_err(|e| e.to_string())?;
    if user.password == password {
        Ok(user)
    } else {
        Err("Invalid credentials".to_string())
    }
}

pub fn handle_create_user(conn: &PgConnection, new_user: User) -> Result<(), String> {
    create_user(conn, &new_user).map_err(|e| e.to_string())?;
    Ok(())
}
