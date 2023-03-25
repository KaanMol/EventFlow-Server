use crate::handlers::error::ResourceError;
use crate::handlers::response::ApiResponse;
use crate::{entities, handlers, AppState, UserClaims};
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
) -> crate::common::Response<EventSource> {
    // TODO: Validate URL
    let id = user_claims.into_inner().cid;

    let new_source = entities::user::EventSource {
        name: body.name.clone(),
        url: body.url.clone(),
    };

    Ok(ApiResponse::from_data(
        handlers::source::create_source(id, new_source, state).await?,
    ))
}

#[actix_web::get("/sync")]
pub async fn sync(
    state: Data<AppState>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<Vec<EventSource>> {
    Ok(ApiResponse::from_data(crate::handlers::source::sync_sources(
        user_claims.into_inner().cid,
        state,
    )
}
