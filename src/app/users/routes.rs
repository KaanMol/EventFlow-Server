use actix_web::{get, post};

use crate::handlers::{error::ResourceError, response::ApiResponse};

#[utoipa::path(
	context_path = "/users",
	tag = "Users",
    responses(
        (status = 200, description = "Authenticated user", body = UserDto),
		(status = 401, description = "Authorization token missing or invalid", body = String)
    )
)]
#[get("")]
async fn read(
    state: actix_web::web::Data<crate::app::State>,
    user_claims: actix_web::web::ReqData<crate::app::UserClaims>,
) -> crate::common::Response<super::dto::UserDto> {
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
    state: actix_web::web::Data<crate::app::State>,
    body: actix_web::web::Json<super::dto::CreateUserDto>,
    user_claims: actix_web::web::ReqData<crate::app::UserClaims>,
) -> crate::common::Response<super::dto::UserDto> {
    let user = crate::handlers::user::create_user(
        crate::entity::user::User {
            id: None,
            name: body.name.clone(),
            identities: vec![user_claims.into_inner().cid],
            sources: vec![],
        },
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(user.into()))
}
