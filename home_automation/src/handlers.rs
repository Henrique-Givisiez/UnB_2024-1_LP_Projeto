use actix_web::{web, HttpResponse, Responder};
use crate::models::{DeviceData, LoginRequest, User};
use crate::controllers::{handle_update_device, handle_login_user, handle_create_user};
use crate::database::DbPool;

pub async fn get_status() -> impl Responder {
    HttpResponse::Ok().body("Sistema de Automação Residencial está rodando.")
}

pub async fn update_device_data(pool: web::Data<DbPool>, data: web::Json<DeviceData>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    match handle_update_device(&conn, data.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Dados do dispositivo atualizados"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn login_user(pool: web::Data<DbPool>, login: web::Json<LoginRequest>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    match handle_login_user(&conn, &login.username, &login.password) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}

pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<User>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    match handle_create_user(&conn, user.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Usuário criado com sucesso"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
