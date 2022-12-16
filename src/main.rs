mod calendar;
mod database;
mod errors;
mod routes;

use actix_web::{web::Data, App, HttpServer};
use ns_scraper::{route::Coordinate, route_builder::RouteFinderBuilder};

use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::sync::*;

#[derive(Clone)]
struct AppState {
    pub db: mongodb::Database,
    pub client: mongodb::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create database connection
    let client_options =
        mongodb::options::ClientOptions::parse("mongodb://root:example@localhost:27017")
            .await
            .unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database("calendarserver");

    // Initialise the app state for Actix
    let state = AppState { client, db };

    // Create the Actix app
    let app = move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(state.clone()))
        // .service(routes::user::create)
        // .service(routes::user::read_all)
        // .service(routes::user::read)
        // .service(routes::calendar::create)
        // .service(routes::calendar::read_for_user)
    };

    // Start the Actix server
    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
