use actix_web::{get, post, put, web::Json};

use crate::{
    app::{
        users::dto::{CreateUserDto, UserDto},
        AppState, UserClaims,
    },
    common::Response,
    entity,
    handlers::{self, error::ResourceError, response::ApiResponse},
};

use super::dto::UpdateUserDto;

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
    user_claims: UserClaims,
) -> crate::common::Response<UserDto> {
    let user = crate::handlers::user::create_user(
        crate::entity::user::User {
            id: None,
            name: body.name.clone(),
            auth_id: user_claims.into_inner().cid,
            sources: vec![],
        },
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(user.into()))
}

#[put("")]
async fn update(
    state: AppState,
    body: Json<UpdateUserDto>,
    user_claims: UserClaims,
) -> crate::common::Response<UserDto> {
    let original_user =
        handlers::user::get_user(user_claims.cid.to_string(), state.clone()).await?;

    let updated_user = entity::user::User {
        id: None,
        name: body.name.clone().unwrap_or(original_user.name),
        auth_id: user_claims.into_inner().cid,
        sources: match body.sources.clone() {
            Some(sources) => sources.into_iter().map(|s| s.into()).collect(),
            None => original_user.sources,
        },
    };

    let updated_user = handlers::user::update_user(updated_user, state).await?;
    Ok(ApiResponse::from_data(updated_user.into()))
}
