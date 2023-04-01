use crate::{entity, tests};

#[actix_rt::test]
pub async fn get_events() {
    let state = tests::setup().await;
    let user_id: String = "get_events".to_string();

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
pub async fn get_event() {
    let state = tests::setup().await;
    let user_id: String = "get_event".to_string();

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_one(tests::data::events(&user_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.get(0).unwrap().title, "Example Event");
}

#[actix_rt::test]
pub async fn update_event_without_id() {
    let state = tests::setup().await;
    let user_id: String = "update_event_without_id".to_string();

    let event = entity::event::EventEntity {
        id: None,
        title: "Example Event".to_string(),
        description: "This is an example event".to_string(),
        start: chrono::Utc::now(),
        end: chrono::Utc::now(),
        location: "Example Location".to_string(),
        all_day: false,
        user_id: user_id.clone(),
        event_uid: None,
    };

    let result =
        crate::handlers::events::update(event, actix_web::web::Data::new(state.clone())).await;

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Input is invalid: `id`");
}

#[actix_rt::test]
pub async fn update_event_with_invalid_id() {
    let state = tests::setup().await;
    let user_id: String = "update_event_with_invalid_id".to_string();

    let event = entity::event::EventEntity {
        id: Some(
            bson::oid::ObjectId::parse_str("642873f8fdfb5999d4794e56")
                .expect("Could not parse test ID"),
        ),
        title: "Example Event".to_string(),
        description: "This is an example event".to_string(),
        start: chrono::Utc::now(),
        end: chrono::Utc::now(),
        location: "Example Location".to_string(),
        all_day: false,
        user_id: user_id.clone(),
        event_uid: None,
    };

    let result =
        crate::handlers::events::update(event, actix_web::web::Data::new(state.clone())).await;

    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().to_string(),
        "The resource with id `642873f8fdfb5999d4794e56` is not found"
    );
}

#[actix_rt::test]
async fn update_event() {
    let state = tests::setup().await;
    let user_id: String = "update_event".to_string();

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_one(tests::data::events(&user_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.len(), 1);

    let event_id = events
        .get(0)
        .unwrap()
        .id
        .clone()
        .expect("Event ID is missing");

    let event = entity::event::EventEntity {
        id: Some(event_id.clone()),
        title: "Updated Event".to_string(),
        description: "This is an updated event".to_string(),
        start: chrono::Utc::now(),
        end: chrono::Utc::now(),
        location: "Updated Location".to_string(),
        all_day: false,
        user_id: user_id.clone(),
        event_uid: None,
    };

    crate::handlers::events::update(event, actix_web::web::Data::new(state.clone()))
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.len(), 1);
    assert_eq!(events.get(0).unwrap().title, "Updated Event");
    assert_eq!(
        events.get(0).unwrap().description,
        "This is an updated event"
    );
    assert_eq!(events.get(0).unwrap().location, "Updated Location");
}

#[actix_rt::test]
pub async fn delete_event() {
    let state = tests::setup().await;
    let user_id: String = "delete_event".to_string();

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_one(tests::data::events(&user_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.len(), 1);

    let event_id = events
        .get(0)
        .unwrap()
        .id
        .clone()
        .expect("Event ID is missing");

    crate::handlers::events::delete(event_id, actix_web::web::Data::new(state.clone()))
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.len(), 0);
}
