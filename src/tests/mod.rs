use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{Service, ServiceFactory, ServiceRequest, ServiceResponse},
    http::Error,
    App, HttpMessage,
};
use jsonwebtoken::TokenData;

use crate::{app, common, entity};

mod app;
pub mod data;
pub mod handlers;

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
            req.extensions_mut().insert(TestClaims {
                cid: user_id.clone(),
            });

            srv.call(req)
        })
}
