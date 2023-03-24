use actix_web::{get, post, web::Json};

use crate::handlers::error::ResourceError;
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
    // user_claims: UserClaims, TODO: Add user claims
) -> crate::common::Response<EventsSourceDto> {
    let source = create_source(
        "je moedfer".to_string(),
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
pub async fn sync(state: AppState) -> crate::common::Response<()> {
    let user = crate::handlers::user::get_user("je moedfer".to_string(), state).await?;
    todo!()
}
