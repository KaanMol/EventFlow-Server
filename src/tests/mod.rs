use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{Service, ServiceFactory, ServiceRequest, ServiceResponse},
    http::Error,
    App, HttpMessage,
};
use jsonwebtoken::TokenData;

use crate::{
    app::{self, Claims, UserClaims},
    common, entity,
};

pub mod data;
pub mod handlers;
mod integration;

pub async fn setup() -> app::State {
    dotenv::dotenv().ok();

    app::State {
        db: common::database::connect_testdb().await,
    }
}

pub struct TestClaims {
    pub cid: String,
}

pub fn get_integration_app(
    state: app::State,
    user_id: String,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(actix_web::web::Data::new(state))
        .wrap_fn(move |req, srv| {
            req.extensions_mut().insert(Claims {
                cid: user_id.clone(),
                name: "John Doe".to_string(),
                nickname: "John".to_string(),
                preferred_username: "John".to_string(),
                given_name: "John".to_string(),
                auth_time: 0,
                iat: 0,
                exp: 0,
                sub: "".to_string(),
                groups: vec!["testGroup".to_string()],
            });

            srv.call(req)
        })
}
