use actix_web::{App, HttpServer};
use std::net::TcpListener;
use dotenv_codegen::dotenv;

mod db;
#[path = "services/user_service.rs"] mod user_service;

#[allow(unused)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_db_connection().await;
    db::run_migrations(&pool);
    const PORT: &str = dotenv!("PORT");
    const HOST: &str = dotenv!("HOST");
    let listener =
        TcpListener::bind(format!("{}:{}", HOST, PORT))
            .expect("Failed to create listener");
    println!("Listening on {}:{}", HOST, PORT);
    HttpServer::new(move || App::new()
        .data(pool.clone())
        .configure(user_service::init_user_routes))
        .listen(listener)?
        .run()
        .await
}
