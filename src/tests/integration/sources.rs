use crate::{app::sources::dto::CreateEventSourceDto, handlers, tests};
use actix_web::{
    test,
    web::{self, Bytes},
    App,
};

#[actix_web::test]
async fn test_add_source() {
    let auth_id = "test_add_source".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::sources::routes::create)),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(CreateEventSourceDto {
            name: "test".to_string(),
            url: "http://test.com".to_string(),
        })
        .to_request();
    let response: handlers::response::ApiResponse<crate::app::sources::dto::EventsSourceDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.error.is_none());
    assert!(response.data.is_some());

    let data = response.data.unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.url, "http://test.com");
}

#[actix_web::test]
async fn test_add_source_with_invalid_url() {
    let auth_id = "test_add_source_with_invalid_url".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::sources::routes::create)),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(CreateEventSourceDto {
            name: "test".to_string(),
            url: "123".to_string(),
        })
        .to_request();
    let response: handlers::response::ApiResponse<crate::app::sources::dto::EventsSourceDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.error.is_some());
    assert!(response.data.is_none());
    assert_eq!(response.error.unwrap().code, 400);
}

#[actix_web::test]
async fn test_add_source_with_invalid_name() {
    let auth_id = "test_add_source_with_invalid_url".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::sources::routes::create)),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(CreateEventSourceDto {
            name: "".to_string(),
            url: "http://test.com".to_string(),
        })
        .to_request();
    let response: handlers::response::ApiResponse<crate::app::sources::dto::EventsSourceDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.error.is_some());
    assert!(response.data.is_none());
    assert_eq!(response.error.unwrap().code, 400);
}
