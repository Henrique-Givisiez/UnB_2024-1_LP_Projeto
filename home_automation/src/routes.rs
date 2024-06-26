use actix_web::web;
use crate::handlers::{get_status, update_device_data, login_user, create_user};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/status")
            .route(web::get().to(get_status))
    )
    .service(
        web::resource("/update")
            .route(web::post().to(update_device_data))
    )
    .service(
        web::resource("/login")
            .route(web::post().to(login_user))
    )
    .service(
        web::resource("/create_user")
            .route(web::post().to(create_user))
    );
}
