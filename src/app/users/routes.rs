use actix_web::{get, post, web::Json};

use crate::{
    app::{
        users::dto::{CreateUserDto, UserDto},
        AppState, UserClaims,
    },
    common::Response,
    handlers::{error::ResourceError, response::ApiResponse},
};

#[utoipa::path(
	context_path = "/users",
	tag = "Users",
    responses(
        (status = 200, description = "Authenticated user", body = UserDto),
		(status = 401, description = "Authorization token missing or invalid", body = String)
    )
)]
#[get("")]
async fn read(state: AppState, user_claims: UserClaims) -> Response<UserDto> {
    let user_identity = user_claims.into_inner().cid;

    let user = crate::handlers::user::get_user(user_identity.clone(), state)
        .await
        .map_err(|_| ResourceError::NotFoundById(user_identity))?;

    Ok(ApiResponse::from_data(user.into()))
}

#[utoipa::path(
	context_path = "/users",
	request_body = CreateUserDto,
	tag = "Users",
    responses(
        (status = 200, description = "Create user", body = UserDto),
		(status = 401, description = "Authorization token missing or invalid", body = String)
    )
)]
#[post("")]
async fn create(
    state: AppState,
    body: Json<CreateUserDto>,
    //user_claims: UserClaims,
) -> crate::common::Response<UserDto> {
    let user = crate::handlers::user::create_user(
        crate::entity::user::User {
            id: None,
            name: body.name.clone(),
            identities: vec!["je moedfer".to_string()],
            sources: vec![],
        },
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(user.into()))
}
