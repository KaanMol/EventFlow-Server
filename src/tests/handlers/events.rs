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