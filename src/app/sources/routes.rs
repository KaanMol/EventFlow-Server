use actix_web::{get, post, web::Json};

use crate::{
    app::{
        sources::dto::{CreateEventSourceDto, EventsSourceDto},
        AppState, UserClaims,
    },
    common::Response,
    handlers::{error::ResourceError, response::ApiResponse},
};

#[post("")]
pub async fn create(
    state: AppState,
    body: Json<CreateEventSourceDto>,
    user_claims: UserClaims,
) -> crate::common::Response<EventsSourceDto> {
    todo!()
}
