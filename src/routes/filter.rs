use crate::handlers::response::ApiResponse;
use crate::{entities, AppState, UserClaims};
use actix_web::web::{Data, Json, ReqData};

#[derive(serde::Deserialize, Clone)]
pub struct CreateFilterBody {
    field: String,
    operator: String,
    value: String,
    url: String,
    calendar_id: String,
}

// FIXME: Proper return type instead of UpdateResult
#[actix_web::post("")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateFilterBody>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<entities::user::CalendarEventSourceFilters> {
    // TODO: Validate Calendar ID

    let user_identity = user_claims.into_inner().cid;

    let new_filter = entities::user::CalendarEventSourceFilters {
        field: body.field.clone(),
        operator: body.operator.clone(),
        value: body.value.clone(),
        calendar_id: body.calendar_id.clone(),
    };

    Ok(ApiResponse::from_data(
        crate::handlers::filter::create_filter(user_identity, body.url.clone(), new_filter, state)
            .await?,
    ))
}
