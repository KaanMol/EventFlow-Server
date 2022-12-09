use crate::{
    routes::{OptionConverter, ResultConverter},
    AppState,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use entity::user as User;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    name: String,
}

#[actix_web::post("/user")]
pub async fn create(state: Data<AppState>, body: Json<CreateUserBody>) -> HttpResponse {
    User::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
        name: ActiveValue::Set(body.name.clone()),
    }
    .insert(&state.database)
    .await
    .reply()
}

#[actix_web::get("/user")]
pub async fn read_all(state: Data<AppState>) -> HttpResponse {
    User::Entity::find().all(&state.database).await.reply()
}

#[actix_web::get("/user/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> HttpResponse {
    User::Entity::find_by_id(user_id.clone())
        .one(&state.database)
        .await
        .reply_option("Could not find user")
}

// TODO: Delete user
