use crate::entity::user::CalendarEventSource;
use crate::handlers::error::ResourceError;
use crate::handlers::response::ApiResponse;
use crate::{entity, handlers, AppState, UserClaims};
use actix_web::web::{Data, Json, ReqData};

#[derive(serde::Deserialize, Clone)]
pub struct CreateSourceBody {
    name: String,
    url: String,
}

#[actix_web::post("")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateSourceBody>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<CalendarEventSource> {
    // TODO: Validate URL
    let id = user_claims.into_inner().cid;

    let new_source = entity::user::CalendarEventSource {
        name: body.name.clone(),
        url: body.url.clone(),
        filters: vec![],
        modifiers: vec![],
    };

    Ok(ApiResponse::from_data(
        handlers::source::create_source(id, new_source, state).await?,
    ))
}

#[actix_web::get("")]
pub async fn read(
    state: Data<AppState>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<Vec<CalendarEventSource>> {
    let user_identity = user_claims.into_inner().cid;
    let user = crate::handlers::user::get_user(user_identity.clone(), state)
        .await
        .map_err(|_| ResourceError::NotFoundById(user_identity))?;

    Ok(ApiResponse::from_data(user.sources))
}
