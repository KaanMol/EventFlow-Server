use crate::{dto::{
	user::UserDto,
	calendar::{EventSourceDto, EventSourceFilterDto, EventSourceModifierDto}
}, common::SecurityAddon};
use crate::handlers::response::ApiResponse;
use crate::UserClaims;
use crate::{
    entity::{self, user::User},
    handlers::error::ResourceError,
    AppState,
};
use actix_web::get;
use actix_web::web::{Data, Json, ReqData};
use utoipa::OpenApi;

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    name: String,
}

#[derive(OpenApi)]
#[openapi(
        paths(
    		read
        ),
		components(
			schemas(
				UserDto, 
				EventSourceDto,
				EventSourceFilterDto,
				EventSourceModifierDto
			)
		),
        tags(
            (name = "Users", description = "Users management endpoint")
        ),
		modifiers(&SecurityAddon)
    )]
pub struct UserApiDoc;

#[actix_web::post("")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<User> {
    let user = crate::handlers::user::create_user(
        entity::user::User {
            id: None,
            name: body.name.clone(),
            identities: vec![user_claims.into_inner().cid],
            sources: vec![],
        },
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(user))
}

#[utoipa::path(
	context_path = "/users",
	tag = "Users",
    responses(
        (status = 200, description = "Authenticated user", body = [UserDto])
    )
)]
#[get("")]
pub async fn read(
    state: Data<AppState>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<UserDto> {
    let user_identity = user_claims.into_inner().cid;
    let user = crate::handlers::user::get_user(user_identity.clone(), state)
        .await
        .map_err(|_| ResourceError::NotFoundById(user_identity))?;

    Ok(ApiResponse::from_data(user.into()))
}

// // TODO: Update user

// // TODO: Delete user
