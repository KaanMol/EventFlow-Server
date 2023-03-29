use crate::entity;

#[actix_rt::test]
pub async fn get_events() {
    let state = super::setup().await;
    let user_id: String = "get_events".to_string();

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_many(super::data::events(&user_id), None)
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(events.get(1).unwrap().title, "Second example Event");

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .delete_many(
            bson::doc! {
                "user_id": user_id
            },
            None,
        )
        .await
        .unwrap();
}

#[actix_rt::test]
pub async fn get_event() {
    let state = super::setup().await;
    let user_id: String = "get_event".to_string();

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .insert_one(super::data::events(&user_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let events =
        crate::handlers::events::get_all(user_id.clone(), actix_web::web::Data::new(state.clone()))
            .await
            .unwrap();

    assert_eq!(events.get(0).unwrap().title, "Example Event");

    state
        .db
        .collection::<entity::event::EventEntity>("events")
        .delete_one(
            bson::doc! {
                "user_id": user_id
            },
            None,
        )
        .await
        .unwrap();
}
