use actix_web::{web, HttpResponse, Responder};                                                               // Crate necessário para criar respostas HTTP
use crate::models::{LoginRequest, NewDevice, NewUser, ReadDevice};                                           // estruturas de dados necessárias
use crate::controllers::{handle_create_device, handle_create_user, handle_login_user, handle_update_device}; // controladores que irão de fato lidar com a lógica
use crate::database::DbPool;                                                                                 // pool de conexões      
use serde_json::Value;

// Apenas verifica o status do sistema
pub async fn get_status() -> impl Responder {
    // RESPOSTA HTTP 200 OK
    HttpResponse::Ok().body("Sistema de Automação Residencial está rodando.")
}

// Cria um novo dispositivo
pub async fn create_device(pool: web::Data<DbPool>, new_device: web::Json<NewDevice>) -> impl Responder {
    // Estabelece uma conexão com o banco de dados
    let conn = pool.get().expect("couldn't get db connection from pool");
    // Chama a função "handle_create_device" contida em "controllers.rs" para lidar com a criação do novo dispositivo
    // parâmetro "data": vai conter o nome e o status do dispositivo
    match handle_create_device(&conn, new_device.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Dispositivo criado com sucesso"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// Atualiza os dados do dispositivo
pub async fn update_device_data(
    pool: web::Data<DbPool>, 
    item: web::Json<Value>
) -> HttpResponse {
    // Verifica se foi passado o id do dispositivo no request
    if item.get("id").is_none() {
        return HttpResponse::BadRequest().json("Dispositivo não encontrado");
    }

    // Tente desserializar para ReadDevice
    let device: ReadDevice = match serde_json::from_value(item.into_inner()) {
        Ok(dev) => dev,
        Err(_) => return HttpResponse::BadRequest().json("Dados inválidos"),
    };

    // Estabelece conexão com o banco de dados
    let conn = pool.get().expect("couldn't get db connection from pool");
    // Chama a função "handle_update_device" para atualizar o dispositivo
    match handle_update_device(&conn, device) {
        Ok(_) => HttpResponse::Ok().json("Dispositivo atualizado com sucesso"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

// Função de login do usuário
pub async fn login_user(pool: web::Data<DbPool>, login: web::Json<LoginRequest>) -> impl Responder {
    // Estabelece uma conexão com o banco de dados
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    // Chama a função "handle_login_user" contida em "controller.rs" para lidar com o login
    match handle_login_user(&conn, &login.email, &login.password) {
        Ok(user) => HttpResponse::Ok().json(user), // RESPOSTA HTTP 200 OK
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}

// Função para criar o usuário (cadastro)
pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
    // Estabelece uma conexão com o banco de dados
    let conn = pool.get().expect("couldn't get db connection from pool");
    // Chama a função "handle_create_user" contida em "controllers.rs" para lidar com o cadastro
    match handle_create_user(&conn, new_user.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Usuário criado com sucesso"), // RESPOSTA HTTP 200 OK
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
