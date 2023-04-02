use crate::{entity, handlers, tests};

#[actix_rt::test]
pub async fn when_get_user_ok_expect_user() {
    let state = tests::setup().await;
    let auth_id = "when_get_user_ok_expect_user".to_string();

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
