use actix_web::guard::{self, Guard};

pub fn get_login_guard() -> impl Guard {
    return guard::Header("Host", "127.0.0.1:8080");
}
