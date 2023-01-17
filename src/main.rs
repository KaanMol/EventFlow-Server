mod calendar;
mod common;
mod entity;
pub mod handlers;
mod routes;

#[cfg(test)]
mod tests;

use actix_web::{web::Data, App, HttpServer, Responder};
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
}

#[actix_web::get("/ping")]
pub async fn ping() -> impl Responder {
    "pong"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create database connection
    let db = common::database::connect().await;

    // Initialise the app state for Actix
    let state = AppState { db };

    // let cors = actix_cors::Cors::default()
    //     .allowed_origin("https://www.rust-lang.org")
    //     .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
    //     .allowed_methods(vec!["GET", "POST"])
    //     .max_age(3600);

    // Create the Actix app
    let app = move || {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(state.clone()))
            .service(ping)
            .service(routes::user::create)
            .service(routes::user::create)
            .service(routes::user::read)
            .service(routes::source::create)
            .service(routes::source::read)
            .service(routes::filter::create)
            .service(routes::modifiers::create)
        // .service(routes::calendar::create)
        // .service(routes::calendar::read_for_user)
    };

    // Start the Actix server
    HttpServer::new(app).bind(("0.0.0.0", 3000))?.run().await
}
