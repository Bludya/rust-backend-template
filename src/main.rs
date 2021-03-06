use actix_web::{App, HttpServer};

mod handlers;

// struct AppState {
//     app_name: String,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(handlers::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
