use actix_web::{web, Responder};

pub fn config_general(cfg: &mut web::ServiceConfig) {
    let general_scope =
        web::scope("/general").service(web::resource("/").route(web::get().to(index)));

    cfg.service(general_scope);
}

async fn index() -> impl Responder {
    "Hello general world!"
}
