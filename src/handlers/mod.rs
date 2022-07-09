use actix_web::web;

mod cadmin;
mod common;
mod general;
mod guards;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(cadmin::config_cadmin)
        .configure(common::config_common)
        .configure(general::config_general);
}
