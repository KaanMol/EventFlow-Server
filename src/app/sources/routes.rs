use actix_web::{get, post, web::Json};

use crate::{common, entity, handlers};

use crate::{
    app::{
        sources::dto::{CreateEventSourceDto, EventsSourceDto},
        AppState, UserClaims,
    },
    handlers::response::ApiResponse,
};

#[post("")]
pub async fn create(
    state: AppState,
    body: Json<CreateEventSourceDto>,
    user_claims: UserClaims,
) -> common::Response<EventsSourceDto> {
    let source = entity::user::EventSource {
        name: body.name.clone(),
        url: body.url.clone(),
    };

    let created_source =
        handlers::source::create_source(user_claims.into_inner().cid, source, state).await?;

    Ok(ApiResponse::from_data(created_source.into()))
}

#[get("/sync")]
pub async fn sync(state: AppState, user_claims: UserClaims) -> crate::common::Response<()> {
    handlers::source::sync_sources(user_claims.into_inner().cid, state).await?;
    Ok(ApiResponse::from_data(()))
}
