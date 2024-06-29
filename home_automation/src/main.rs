// Framework para aplicação web. Fornece um servidor HTTP que irá lidar com as rotas, middlewares e entre outros
// https://actix.rs/docs
use actix_web::{App, HttpServer, middleware, web}; 
// Biblioteca para carregar variáveis de ambiente (URL da database por exemplo) de um arquivo .env para o ambiente de execução da aplicação
// https://crates.io/crates/dotenv
use dotenv::dotenv;
// Biblioteca padrão do Rust para acessar variáveis de ambiente diretamento do sitema operacional ou contêiner
// https://doc.rust-lang.org/nightly/std/env/index.html
use std::env;

#[macro_use]
extern crate diesel;

mod schema;
mod handlers;
mod models;
mod routes;
mod database;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carrega variáveis de ambiente do arquivo .env
    dotenv().ok();
    env_logger::init();

    // Obtém variáveis de ambiente
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não está definida no arquivo .env");
    // Cria a pool de conexões
    let pool = database::establish_connection(&database_url);

    // Inicializa o servidor Actix
    HttpServer::new(move || {
        App::new()
            // Adiciona pool de conexões do banco de dados ao app
            .app_data(web::Data::new(pool.clone()))
            // Configura middlewares
            .wrap(middleware::Logger::default())
            // Configura rotas
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
