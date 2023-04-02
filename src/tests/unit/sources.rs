use crate::{entity, handlers, tests};

#[actix_rt::test]
pub async fn when_add_source_ok_expect_source_added_to_user() {
    let state = tests::setup().await;
    let auth_id = "when_add_source_ok_expect_added_source".to_string();

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let new_source = entity::user::EventSource {
        name: "Example Source".to_string(),
        url: "https://example.com".to_string(),
    };

    let updated_user = handlers::source::create_source(
        auth_id.clone(),
        new_source,
        actix_web::web::Data::new(state.clone()),
    )
    .await
    .unwrap();

    assert_eq!(updated_user.sources.len(), 1);
    assert_eq!(updated_user.sources.get(0).unwrap().name, "Example Source");
}

#[actix_rt::test]
pub async fn when_add_source_with_invalid_name_expect_error() {
    let state = tests::setup().await;
    let auth_id = "when_add_source_with_invalid_name_expect_error".to_string();

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let new_source = entity::user::EventSource {
        name: "".to_string(),
        url: "https://example.com".to_string(),
    };

    let result = handlers::source::create_source(
        auth_id.clone(),
        new_source,
        actix_web::web::Data::new(state.clone()),
    )
    .await;

    assert!(result.is_err());
}

#[actix_rt::test]
pub async fn when_add_source_with_invalid_url_expect_error() {
    let state = tests::setup().await;
    let auth_id = "when_add_source_with_invalid_url_expect_error".to_string();

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let new_source = entity::user::EventSource {
        name: "Example source".to_string(),
        url: "ftp://example.com".to_string(),
    };

    let result = handlers::source::create_source(
        auth_id.clone(),
        new_source,
        actix_web::web::Data::new(state.clone()),
    )
    .await;

    assert!(result.is_err());
}

#[actix_rt::test]
pub async fn when_delete_source_ok_expect_success() {
    let state = tests::setup().await;
    let auth_id = "when_delete_source_ok_expect_success".to_string();

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let new_source = entity::user::EventSource {
        name: "Example Source".to_string(),
        url: "https://example.com".to_string(),
    };

    let user = handlers::source::create_source(
        auth_id.clone(),
        new_source.clone(),
        actix_web::web::Data::new(state.clone()),
    )
    .await
    .unwrap();

    assert_eq!(user.sources.len(), 1);

    let updated_user = handlers::source::delete_source(
        auth_id.clone(),
        new_source.clone(),
        actix_web::web::Data::new(state.clone()),
    )
    .await
    .unwrap();

    assert_eq!(updated_user.sources.len(), 0);
}
