use actix_web::{get, post, web::Json};

use crate::handlers::source::create_source;

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
) -> crate::common::Response<EventsSourceDto> {
    let source = create_source(
        user_claims.cid.clone(),
        crate::entity::user::EventSource {
            name: body.name.clone(),
            url: body.url.clone(),
        },
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(source.into()))
}

#[get("/sync")]
pub async fn sync(state: AppState, user_claims: UserClaims) -> crate::common::Response<()> {
    crate::handlers::source::sync_sources(user_claims.cid.clone(), state).await?;
    Ok(ApiResponse::from_data(()))
}
