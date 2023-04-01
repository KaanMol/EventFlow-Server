use crate::{
    entity,
    handlers::{self},
    tests,
};

#[actix_rt::test]
pub async fn get_user() {
    let state = tests::setup().await;
    let auth_id = "get_user".to_string();

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(tests::data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let user = handlers::user::get_user(auth_id.clone(), actix_web::web::Data::new(state.clone()))
        .await
        .unwrap();

    assert_eq!(user.name, "John Doe");
}
