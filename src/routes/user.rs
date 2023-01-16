use crate::{
    entity::{self, user::User},
    handlers::error::ResourceError,
    AppState,
};
use actix_web::web::{Data, Json, Path};
use mongodb::results::InsertOneResult;

use crate::handlers::response::ApiResponse;

type Response<T> = std::result::Result<ApiResponse<T>, ResourceError>;

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    name: String,
}

#[actix_web::post("/users")]
pub async fn create(state: Data<AppState>, body: Json<CreateUserBody>) -> Response<User> {
    let result = state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(
            entity::user::User {
                id: None,
                name: body.name.clone(),
                sources: vec![],
            },
            None,
        )
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    if let Some(user_id) = result.inserted_id.as_object_id() {
        let user = crate::handlers::user::get_user(user_id, state)
            .await
            .map_err(|_| ResourceError::NotFoundById(user_id.to_string()))?;

        Ok(ApiResponse::from_data(user))
    } else {
        Err(ResourceError::Unknown)
    }
}

#[actix_web::get("/users/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> Response<User> {
    let id = crate::routes::parse_id(&user_id)?;

    let user = crate::handlers::user::get_user(id, state)
        .await
        .map_err(|_| ResourceError::NotFoundById(id.to_string()))?;

    Ok(ApiResponse::from_data(user))
}

// // TODO: Update user

// // TODO: Delete user
