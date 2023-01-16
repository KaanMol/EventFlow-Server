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

// FIXME: Proper return type instead of InsertOneResult
#[actix_web::post("/users")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Response<InsertOneResult> {
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
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    Ok(ApiResponse::from_data(result))
}

#[utoipa::path(
        get,
        path = "/users/{user_id}",
        responses(
            (status = 200, description = "User found succesfully", body = User),
            (status = 404, description = "User was not found")
        ),
        params(
            ("user_id" = String, Path, description = "User database id to get User for"),
        )
    )]
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
