mod calendar;
mod common;
mod entity;
pub mod handlers;
mod routes;

#[cfg(test)]
mod tests;

use actix_web::{
    dev::ServiceRequest,
    error::ErrorUnauthorized,
    web::{Data, ReqData},
    App, Error, HttpMessage, HttpServer, Responder,
};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    name: String,
    cid: String,
    nickname: String,
    preferred_username: String,
    given_name: String,
    auth_time: i64,
    iat: i64,
    exp: i64,
    sub: String,
    groups: Vec<String>,
}

#[actix_web::get("/ping")]
pub async fn ping() -> impl Responder {
    "pong"
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let result = decode::<UserClaims>(
        credentials.token(),
        // TODO: move this to documentation
        // generate correct pem with:
        // openssl rsa -pubout -in rsa-private.pem -out rsa-public.pem
        &DecodingKey::from_rsa_pem(include_bytes!("certs/token.pem")).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .map_err(|e| ErrorUnauthorized(e.to_string()));

    match result {
        Ok(user_claims) => {
            // TODO: Implement permissions in Identity Provider to make Authoriation possible
            // req.attach(claims.permissions);

            // Inject user claims in requests
            req.extensions_mut().insert(user_claims.claims);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
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
        // Initialise the JWT validator middleware
        let auth = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .wrap(auth)
            .app_data(Data::new(state.clone()))
            .service(ping)
            .service(
                actix_web::web::scope("/users")
                    .service(routes::user::create)
                    .service(routes::user::read),
            )
            .service(
                actix_web::web::scope("/sources")
                    .service(routes::source::create)
                    .service(routes::source::read),
            )
            .service(actix_web::web::scope("/filters").service(routes::filter::create))
            .service(actix_web::web::scope("/modifiers").service(routes::modifiers::create))
    };

    // Start the Actix server
    HttpServer::new(app).bind(("0.0.0.0", 3000))?.run().await
}
