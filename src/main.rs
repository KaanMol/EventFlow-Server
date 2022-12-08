mod calendar;
mod database;
mod errors;
mod routes;

use actix_web::{web::Data, App, HttpServer};

#[derive(Clone)]
pub struct AppState {
    pub database: sea_orm::DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create a connection pool to the database
    let database = database::Database::connect()
        .await
        .expect("could not connect to database"); // TODO: handle this error

    // Initialise the app state for Actix
    let state = AppState { database };

    // Create the Actix app
    let app = move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(routes::user::create_user)
            .service(routes::user::get_all_users)
            .service(routes::user::get_user_by_id)
    };

    // Start the Actix server
    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
