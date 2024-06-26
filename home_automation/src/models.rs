use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
    pub device_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceData {
    pub device_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
