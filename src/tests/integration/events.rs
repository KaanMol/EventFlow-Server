use crate::{app::events::dto::UpdateEventDto, entity, handlers, tests};
use actix_web::{test, web};

#[actix_web::test]
async fn test_update_non_existing_event() {
    let auth_id = "test_update_non_existing_event".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::events::routes::update)),
    )
    .await;

    let request = test::TestRequest::put()
        .uri("/")
        .set_json(UpdateEventDto {
            id: "6421b6fe892176073d63919b".to_string(),
            title: None,
            description: None,
            start: None,
            end: None,
            location: None,
            all_day: None,
        })
        .to_request();

    let response: handlers::response::ApiResponse<crate::app::events::dto::EventDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.data.is_none());
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, 404);
}

#[actix_web::test]
async fn test_update_event() {
    let auth_id = "test_update_event".to_string();
    let state = tests::setup().await;
    let app = tests::get_integration_app(state.clone(), auth_id.clone());

    let service = test::init_service(
        app.service(web::scope("/").service(crate::app::events::routes::update)),
    )
    .await;

    let inserted_id = state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_one(tests::data::events(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap()
        .inserted_id
        .as_object_id()
        .unwrap();

    let request = test::TestRequest::put()
        .uri("/")
        .set_json(UpdateEventDto {
            id: inserted_id.to_string(),
            title: Some("New title".to_string()),
            description: Some("New description".to_string()),
            start: None,
            end: None,
            location: Some("New location".to_string()),
            all_day: None,
        })
        .to_request();

    let response: handlers::response::ApiResponse<crate::app::events::dto::EventDto> =
        test::call_and_read_body_json(&service, request).await;

    assert!(response.error.is_none());
    assert!(response.data.is_some());

    let body = response.data.unwrap();

    assert_eq!(body.title, "New title");
    assert_eq!(body.description, "New description");
    assert_eq!(body.location, "New location");
}
