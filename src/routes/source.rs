use crate::{entity, AppState};
use actix_web::{
    error,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;
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
