use crate::{
    app::{self, sources::dto::EventsSourceDto},
    common, handlers,
};
use actix_web::{delete, get, post, web::Json};

use crate::{
    app::{AppState, UserClaims},
    handlers::response::ApiResponse,
};

#[post("")]
pub async fn create(
    body: Json<EventsSourceDto>,
    state: AppState,
    user_claims: UserClaims,
) -> common::Response<app::users::dto::UserDto> {
    let result = handlers::source::create_source(
        user_claims.into_inner().cid,
        body.into_inner().into(),
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(result.into()))
}

#[delete("")]
pub async fn delete(
    body: Json<EventsSourceDto>,
    state: AppState,
    user_claims: UserClaims,
) -> common::Response<app::users::dto::UserDto> {
    let result = handlers::source::delete_source(
        user_claims.into_inner().cid,
        body.into_inner().into(),
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(result.into()))
}

#[get("/sync")]
pub async fn sync(state: AppState, user_claims: UserClaims) -> crate::common::Response<()> {
    handlers::source::sync_sources(user_claims.into_inner().cid, state).await?;
    Ok(ApiResponse::from_data(()))
}
