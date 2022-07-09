use actix_web::{web, Responder};

pub fn config_common(cfg: &mut web::ServiceConfig) {
    let common_scope =
        web::scope("/common").service(web::resource("/").route(web::get().to(index)));

    cfg.service(common_scope);
}

async fn index() -> impl Responder {
    "Hello common world!"
}
