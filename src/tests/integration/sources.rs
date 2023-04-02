use crate::{
    app::{self, sources::dto::EventsSourceDto},
    entity, handlers, tests,
};
use actix_web::{test, web};

#[actix_web::test]
async fn when_add_source_ok_expect_user_with_new_source() {
    let auth_id = "when_add_source_ok_expect_user_with_new_source".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::sources::routes::create)),
    )
    .await;

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(EventsSourceDto {
            name: "test".to_string(),
            url: "http://test.com".to_string(),
        })
        .to_request();
    let response: handlers::response::ApiResponse<app::users::dto::UserDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.error.is_none());
    assert!(response.data.is_some());

    let body = response.data.unwrap();
    let source = body.sources.get(0).unwrap();

    assert_eq!(source.name, "test");
    assert_eq!(source.url, "http://test.com");
}

#[actix_web::test]
async fn when_add_source_with_invalid_url_expect_error() {
    let auth_id = "when_add_source_with_invalid_url_expect_error".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::sources::routes::create)),
    )
    .await;

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(EventsSourceDto {
            name: "test".to_string(),
            url: "123".to_string(),
        })
        .to_request();
    let response: handlers::response::ApiResponse<crate::app::users::dto::UserDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.error.is_some());
    assert!(response.data.is_none());
    assert_eq!(response.error.unwrap().code, 400);
}

#[actix_web::test]
async fn when_add_source_with_invalid_name_expect_error() {
    let auth_id = "when_add_source_with_invalid_name_expect_error".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::sources::routes::create)),
    )
    .await;

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(EventsSourceDto {
            name: "".to_string(),
            url: "http://test.com".to_string(),
        })
        .to_request();
    let response: handlers::response::ApiResponse<crate::app::users::dto::UserDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.error.is_some());
    assert!(response.data.is_none());
    assert_eq!(response.error.unwrap().code, 400);
}
