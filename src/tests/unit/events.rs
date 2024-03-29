use std::str::FromStr;

use bson::oid::ObjectId;

use crate::{entity, handlers::error::ResourceError, tests};

#[actix_rt::test]
pub async fn when_get_events_ok_expect_events() {
    let state = tests::setup().await;
    let user_id: String = "when_get_events_ok_expect_events".to_string();

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
pub async fn when_get_events_empty_expect_empty_list() {
    let state = tests::setup().await;
    let user_id: String = "test_get_events_empty".to_string();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert!(events.is_empty());
}

#[actix_rt::test]
pub async fn when_get_event_ok_expect_event() {
    let state = tests::setup().await;
    let user_id: String = "when_get_event_ok_expect_event".to_string();

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
pub async fn when_get_not_existing_event_expect_error() {
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

#[actix_rt::test]
pub async fn when_update_event_ok_expect_success() {
    let state = tests::setup().await;
    let auth_id = "when_update_event_ok_expect_success".to_string();

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let event_id = state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_one(tests::data::events(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap()
        .inserted_id
        .as_object_id()
        .unwrap();

    let event = crate::handlers::events::update(
        entity::event::EventEntity {
            id: Some(event_id.clone()),
            title: "Updated Event".to_string(),
            description: "Updated Description".to_string(),
            location: "Updated Location".to_string(),
            start: chrono::Utc::now(),
            end: chrono::Utc::now(),
            user_id: auth_id.clone(),
            all_day: false,
            event_uid: None,
        },
        actix_web::web::Data::new(state.clone()),
    )
    .await
    .unwrap();

    assert_eq!(event.title, "Updated Event");
    assert_eq!(event.description, "Updated Description");
    assert_eq!(event.location, "Updated Location");
}
