use actix_web::{get, post};

use crate::{
    app::{AppState, UserClaims},
    common::Response,
    handlers::response::ApiResponse,
};

#[utoipa::path(
	context_path = "/events",
	tag = "Events",
    responses(
        (status = 200, description = "Authenticated user", body = UserDto),
		(status = 401, description = "Authorization token missing or invalid", body = String)
    )
)]
#[get("")]
pub async fn read(state: AppState, user_claims: UserClaims) -> Response<String> {
    // let user_identity = user_claims.into_inner().cid;

    // let user = crate::handlers::user::get_user(user_identity.clone(), state)
    //     .await
    //     .map_err(|_| ResourceError::NotFoundById(user_identity))?;

    Ok(ApiResponse::from_data("Hiii".to_string()))
}

#[utoipa::path(
	context_path = "/events",
	tag = "Events",
    responses(
        (status = 200, description = "Authenticated user", body = UserDto),
		(status = 401, description = "Authorization token missing or invalid", body = String)
    )
)]
#[post("")]
pub async fn create(state: AppState, user_claims: UserClaims) -> Response<String> {
    // Get the unique user ID from the claims
    let user_identity = user_claims.into_inner().cid;

    // Retrieve the user from the database
    let user = crate::handlers::user::get_user(user_identity.clone(), state)
        .await
        .map_err(|_| ResourceError::NotFoundById(user_identity))?;

    // Return the response
    Ok(ApiResponse::from_data("hi".to_string()))
}
