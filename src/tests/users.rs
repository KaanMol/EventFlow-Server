use crate::{
    entity,
    handlers::{self},
};

use super::data;

#[actix_rt::test]
pub async fn get_user() {
    let state = super::setup().await;
    let auth_id = "get_user".to_string();

    state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(data::users(&auth_id).get(0).unwrap(), None)
        .await
        .unwrap();

    let user =
        handlers::user::get_user(auth_id.clone(), actix_web::web::Data::new(state.clone())).await;

    match user {
        Ok(user) => assert_eq!(user.name, "John Doe"),
        Err(error) => panic!("{}", error),
    }

    state
        .db
        .collection::<entity::user::User>("users")
        .delete_one(
            bson::doc! {
                "auth_id": auth_id
            },
            None,
        )
        .await
        .unwrap();
}
