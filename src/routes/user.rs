use crate::AppState;
use actix_web::{
    error,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use entity::user as User;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use uuid::Uuid;

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    name: String,
}

#[actix_web::post("/users")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Result<HttpResponse, Error> {
    let user = User::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4().to_string()),
        name: ActiveValue::Set(body.name.clone()),
    }
    .insert(&state.database)
    .await
    .map_err(|_| error::ErrorBadRequest("Could not create user"))?;

    Ok(HttpResponse::Created().json(user))
}

#[actix_web::get("/users")]
pub async fn read_all(state: Data<AppState>) -> Result<HttpResponse, Error> {
    let users = User::Entity::find()
        .all(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?;

    Ok(HttpResponse::Ok().json(users))
}

#[actix_web::get("/users/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> Result<HttpResponse, Error> {
    let user = User::Entity::find_by_id(user_id.clone())
        .one(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?
        .ok_or_else(|| error::ErrorNotFound(format!("Could not find user with id {}", user_id)))?;

    Ok(HttpResponse::Ok().json(user))
}

// TODO: Update user

// TODO: Delete user
