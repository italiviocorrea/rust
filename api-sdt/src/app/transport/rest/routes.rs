use actix_web::web;
use actix_files as fs;
use crate::app::transport::rest::handlers::{projeto_create, delete_projeto, health_check_handler, find_by_name_projeto, findall_projeto, projeto_update};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("/api/v1/apm")
        .route("/healthy", web::get().to(health_check_handler))
    );
}

pub fn swaggerui_routes(cfg: &mut web::ServiceConfig) {
    cfg.service( fs::Files::new("/api/v1/swaggerui", "./api/v1/swaggerui").show_files_listing());
}

pub fn projeto_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            .route("/projetos", web::post().to(projeto_create))
            .route("/projetos", web::get().to(findall_projeto))
            .route("/projetos/{nome}", web::get().to(find_by_name_projeto))
            .route("/projetos/{nome}", web::delete().to(delete_projeto))
            .route("/projetos/{nome}",web::put().to(projeto_update))
    );
}