use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod events;
mod filters;
mod middleware;
mod routes;
mod rules;
mod sources;
mod users;

#[derive(Clone)]
pub struct State {
    pub db: mongodb::Database,
}

pub type AppState = actix_web::web::Data<State>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
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

pub type UserClaims = actix_web::web::ReqData<Claims>;

pub async fn start() -> Result<(), std::io::Error> {
    // Create database connection
    let db = crate::common::database::connect().await;

    // Initialise the app state for Actix
    let state = State { db };

    let mut open_api = api::RootApiDoc::openapi();
    open_api.merge(events::ApiDoc::openapi());
    open_api.merge(filters::ApiDoc::openapi());
    open_api.merge(rules::ApiDoc::openapi());
    open_api.merge(sources::ApiDoc::openapi());
    open_api.merge(users::ApiDoc::openapi());

    // Start the Actix server
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .service(routes::ping)
            .service(users::routes())
            .service(events::routes())
            // .service(
            // )
            // .service(
            //     actix_web::web::scope("/sources")
            //         .wrap(auth.clone())
            //         .service(routes::source::create)
            //         .service(routes::source::read),
            // )
            // .service(
            //     actix_web::web::scope("/filters")
            //         .wrap(auth.clone())
            //         .service(routes::filter::create),
            // )
            // .service(
            //     actix_web::web::scope("/modifiers")
            //         .wrap(auth.clone())
            //         .service(routes::modifiers::create),
            // )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", open_api.clone()),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
