use actix_web::{web, HttpResponse, Responder};                                         // Crate necessário para criar respostas HTTP
use crate::models::{DeviceData, LoginRequest, User};                                   // estruturas de dados necessárias
use crate::controllers::{handle_update_device, handle_login_user, handle_create_user}; // controladores que irão de fato lidar com a lógica
use crate::database::DbPool;                                                           // pool de conexões

// Apenas verifica o status do sistema
pub async fn get_status() -> impl Responder {
    // RESPOSTA HTTP 200 OK
    HttpResponse::Ok().body("Sistema de Automação Residencial está rodando.")
}

// Atualiza os dados do dispositivo
pub async fn update_device_data(pool: web::Data<DbPool>, data: web::Json<DeviceData>) -> impl Responder {
    // Estabelece uma conexão com o banco de dados
    let conn = pool.get().expect("couldn't get db connection from pool");
    // Chama a função "handle_update_device" contida em "controllers.rs" para lidar com o update
    // parâmetro "data": vai conter o id do dispositivo e o status dele
    match handle_update_device(&conn, data.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Dados do dispositivo atualizados"), // RESPOSTA HTTP 200 OK
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// Função de login do usuário
pub async fn login_user(pool: web::Data<DbPool>, login: web::Json<LoginRequest>) -> impl Responder {
    // Estabelece uma conexão com o banco de dados
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    // Chama a função "handle_login_user" contida em "controller.rs" para lidar com o login
    match handle_login_user(&conn, &login.username, &login.password) {
        Ok(user) => HttpResponse::Ok().json(user), // RESPOSTA HTTP 200 OK
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}

// Função para criar o usuário (cadastro)
pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<User>) -> impl Responder {
    // Estabelece uma conexão com o banco de dados
    let conn = pool.get().expect("couldn't get db connection from pool");
    // Chama a função "handle_create_user" contida em "controllers.rs" para lidar com o cadastro
    match handle_create_user(&conn, user.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Usuário criado com sucesso"), // RESPOSTA HTTP 200 OK
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
