use crate::handlers::response::ApiResponse;
use crate::{
    entity::{self, user::User},
    handlers::error::ResourceError,
    AppState,
};
use actix_web::web::{Data, Json, Path};

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    name: String,
}

#[actix_web::post("/users")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> crate::common::Response<User> {
    let user = crate::handlers::user::create_user(
        entity::user::User {
            id: None,
            name: body.name.clone(),
            sources: vec![],
        },
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(user))
}

#[actix_web::get("/users/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> crate::common::Response<User> {
    let id = crate::routes::parse_id(&user_id)?;

    let user = crate::handlers::user::get_user(id, state)
        .await
        .map_err(|_| ResourceError::NotFoundById(id.to_string()))?;

    Ok(ApiResponse::from_data(user))
}

// // TODO: Update user

// // TODO: Delete user
