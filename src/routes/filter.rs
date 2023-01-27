use crate::handlers::error::ResourceError;
use crate::handlers::response::ApiResponse;
use crate::routes::parse_id;
use crate::{entity, AppState};
use actix_web::web::{Data, Json};

#[derive(serde::Deserialize, Clone)]
pub struct CreateFilterBody {
    field: String,
    operator: String,
    value: String,
    url: String,
    calendar_id: String,
    user_id: String,
}

// FIXME: Proper return type instead of UpdateResult
#[actix_web::post("/filters")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateFilterBody>,
) -> crate::common::Response<entity::user::CalendarEventSourceFilters> {
    // TODO: Validate Calendar ID

    let id = parse_id(&body.user_id)?;

    let new_filter = entity::user::CalendarEventSourceFilters {
        field: body.field.clone(),
        operator: body.operator.clone(),
        value: body.value.clone(),
        calendar_id: body.calendar_id.clone(),
    };

    Ok(ApiResponse::from_data(
        crate::handlers::filter::create_filter(id, body.url.clone(), new_filter, state).await?,
    ))
}
