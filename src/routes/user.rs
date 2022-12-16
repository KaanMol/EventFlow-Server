use crate::{entity, AppState};
use actix_web::{
    error,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use mongodb::bson::oid::ObjectId;
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
    let result = state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(
            entity::user::User {
                name: body.name.clone(),
                sources: vec![],
            },
            None,
        )
        .await
        .map_err(|_| error::ErrorBadRequest("Could not create user"))?;

    Ok(HttpResponse::Created().json(result.inserted_id))
}

#[actix_web::get("/users/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> Result<HttpResponse, Error> {
    let id = ObjectId::parse_str(user_id.as_str())
        .map_err(|_| error::ErrorBadRequest("Invalid user id"))?;

    let user = state
        .db
        .collection::<entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "_id": id
            },
            None,
        )
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?
        .ok_or_else(|| error::ErrorNotFound(format!("Could not find user with id {}", user_id)))?;

    Ok(HttpResponse::Ok().json(user))
}

// // TODO: Update user

// // TODO: Delete user
