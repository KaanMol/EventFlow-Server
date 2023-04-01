use std::str::FromStr;

use bson::oid::ObjectId;

use crate::{entity, handlers::error::ResourceError, tests};

#[actix_rt::test]
pub async fn test_get_events_ok() {
    let state = tests::setup().await;
    let user_id: String = "test_get_events_ok".to_string();

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_many(tests::data::events(&user_id), None)
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(events.get(1).unwrap().title, "Second example Event");
}

#[actix_rt::test]
pub async fn test_get_events_empty() {
    let state = tests::setup().await;
    let user_id: String = "test_get_events_empty".to_string();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert!(events.is_empty());
}

#[actix_rt::test]
pub async fn test_get_event_ok() {
    let state = tests::setup().await;
    let user_id: String = "test_get_event_ok".to_string();

    let event_id = state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_one(tests::data::events(&user_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let event = crate::handlers::events::get_one(
        event_id.inserted_id.as_object_id().unwrap(),
        actix_web::web::Data::new(state.clone()),
    )
    .await
    .unwrap();

    assert_eq!(event.title, "Example Event");
}

#[actix_rt::test]
pub async fn test_get_event_empty() {
    let state = tests::setup().await;
    let not_existing_event_id = ObjectId::from_str("5f1f3c7d7c3c4b0b8c8b4567").unwrap();

    let event = crate::handlers::events::get_one(
        not_existing_event_id,
        actix_web::web::Data::new(state.clone()),
    )
    .await;

    let error = ResourceError::NotFoundById(not_existing_event_id.to_string()).to_string();
    assert!(event.is_err());

    match event {
        Err(e) => assert_eq!(e.to_string(), error),
        _ => assert!(false),
    }
}
