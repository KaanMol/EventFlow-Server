use crate::{entity, handlers, tests};
use actix_web::{test, web};

#[actix_web::test]
async fn when_get_user_ok_expect_user() {
    let auth_id = "when_get_user_ok_expect_user".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let service =
        test::init_service(app.service(web::scope("/").service(crate::app::users::routes::read)))
            .await;

    let request = test::TestRequest::get().uri("/").to_request();
    let response: handlers::response::ApiResponse<crate::app::users::dto::UserDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.data.is_some());
    assert!(response.error.is_none());
    assert_eq!(response.data.unwrap().auth_id, auth_id);
}

#[actix_web::test]
async fn when_get_not_existing_user_ok_expect_not_found() {
    let auth_id = "when_get_not_existing_user_ok_expect_not_found".to_string();
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
