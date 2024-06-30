// Importações necessárias do framework Actix-web e dos manipuladores (handlers) definidos localmente
use actix_web::web;
use crate::handlers::{create_device, create_user, get_status, login_user, update_device_data};

// Função que inicializa e configura as rotas para o serviço web
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    // Configura o endpoint `/status` para responder a requisições GET
    .service(
        web::resource("/status") // Define o caminho do recurso
            .route(web::get().to(get_status)) // Mapeia a rota GET para a função `get_status`
    )
    // Configura o endpoint `/create_device` para responder a requisições POST
    .service(
        web::resource("create_device")
        .route(web::post().to(create_device))
    )
    // Configura o endpoint `/update_device` para responder a requisições POST
    .service(
        web::resource("/update_device") // Define o caminho do recurso
            .route(web::post().to(update_device_data)) // Mapeia a rota POST para a função `update_device_data`
    )
    // Configura o endpoint `/login` para responder a requisições POST
    .service(
        web::resource("/login") // Define o caminho do recurso
            .route(web::post().to(login_user)) // Mapeia a rota POST para a função `login_user`
    )
    // Configura o endpoint `/create_user` para responder a requisições POST
    .service(
        web::resource("/create_user") // Define o caminho do recurso
            .route(web::post().to(create_user)) // Mapeia a rota POST para a função `create_user`
    );
}
