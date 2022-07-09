use actix_web::{web, Responder};

use crate::handlers::guards::get_login_guard;

pub fn config_cadmin(cfg: &mut web::ServiceConfig) {
    let cadmin_scope = web::scope("/cadmin")
        .guard(get_login_guard())
        .service(web::resource("/").route(web::get().to(index)));

    cfg.service(cadmin_scope);
}

async fn index() -> impl Responder {
    "Hello cadmin world!"
}
