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
