use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

mod handlers;
mod models;
mod routes;
mod database;
mod controllers;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carrega variáveis de ambiente do arquivo .env
    dotenv().ok();
    env_logger::init();

    // Obtém variáveis de ambiente
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não está definida no arquivo .env");

    // Cria o gerenciador de conexão do banco de dados
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Falha ao criar pool de conexões");

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
