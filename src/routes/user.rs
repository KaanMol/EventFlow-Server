use crate::handlers::response::ApiResponse;
use crate::UserClaims;
use crate::{
    entity::{self, user::User},
    handlers::error::ResourceError,
    AppState,
};
use actix_web::web::{Data, Json, ReqData};

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    name: String,
}

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

#[actix_web::get("")]
pub async fn read(
    state: Data<AppState>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<User> {
    let user_identity = user_claims.into_inner().cid;
    let user = crate::handlers::user::get_user(user_identity.clone(), state)
        .await
        .map_err(|_| ResourceError::NotFoundById(user_identity))?;

    Ok(ApiResponse::from_data(user))
}

// // TODO: Update user

// // TODO: Delete user
