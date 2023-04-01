use crate::{
    app::AppState,
    common::Response,
    entity,
    handlers::{self},
    tests,
};
use actix_web::{dev::Service, http::header};

use actix_web::{
    http::{self, header::ContentType},
    test,
    web::{self, Bytes},
    App, HttpMessage,
};
use jsonwebtoken::TokenData;

#[actix_web::test]
async fn test_get_user_ok() {
    let auth_id = "test_get_user_ok".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let app =
        test::init_service(app.service(web::scope("/").service(crate::app::users::routes::read)))
            .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp: handlers::response::ApiResponse<crate::app::users::dto::UserDto> =
        test::call_and_read_body_json(&app, req).await;

    let body = resp.data.unwrap();
    assert_eq!(body.auth_id, auth_id);
#[actix_web::test]
async fn test_get_not_existing_user() {
    let auth_id = "test_get_not_existing_user".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service =
        test::init_service(app.service(web::scope("/").service(crate::app::users::routes::read)))
            .await;

    let request = test::TestRequest::get().uri("/").to_request();
    let response: handlers::response::ApiResponse<crate::app::users::dto::UserDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.data.is_none());
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, 404);
}
