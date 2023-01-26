mod calendar;
mod common;
mod entity;
pub mod handlers;
mod routes;

#[cfg(test)]
mod tests;

use actix_web::{
    web::{Data, Json},
    App, HttpServer, Responder,
};
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
}

#[actix_web::get("/ping")]
pub async fn ping() -> impl Responder {
    "pong"
}

#[derive(serde::Deserialize, Clone)]
pub struct AuthToken {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String,
}

#[actix_web::post("/verify")]
pub async fn verify(body: Json<AuthToken>) -> impl Responder {
    let token = decode::<Claims>(
        &body.token,
        // TODO: move this to documentation
        // generate correct pem with:
        // openssl rsa -pubout -in rsa-private.pem -out rsa-public.pem
        &DecodingKey::from_rsa_pem(include_bytes!("certs/token.pem")).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .unwrap();

    actix_web::web::Json(token.claims)
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
            .service(routes::user::read)
            .service(routes::source::create)
            .service(routes::source::read)
            .service(routes::filter::create)
            .service(routes::modifiers::create)
            .service(verify)
        // .service(routes::calendar::create)
        // .service(routes::calendar::read_for_user)
    };

    // Start the Actix server
    HttpServer::new(app).bind(("0.0.0.0", 3000))?.run().await
}
