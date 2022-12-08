use crate::{
    routes::{OptionConverter, ResultConverter},
    AppState,
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use entity::user as User;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

#[derive(serde::Deserialize, Clone)]
struct CreateUserBody {
    name: String,
}

#[actix_web::post("/user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> HttpResponse {
    User::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
        name: ActiveValue::Set(body.name.clone()),
    }
    .insert(&state.database)
    .await
    .reply()
}

#[actix_web::get("/user/all")]
pub async fn get_all_users(state: Data<AppState>) -> HttpResponse {
    User::Entity::find().all(&state.database).await.reply()
}

#[actix_web::get("/user/{user_id}")]
pub async fn get_user_by_id(state: Data<AppState>, user_id: String) -> HttpResponse {
    User::Entity::find_by_id(user_id)
        .one(&state.database)
        .await
        .reply_option()
}
