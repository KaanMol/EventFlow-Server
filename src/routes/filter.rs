use crate::handlers::error::ResourceError;
use crate::handlers::response::ApiResponse;
use crate::routes::{parse_id, to_bson};
use crate::{entity, AppState};
use actix_web::web::{Data, Json};
use mongodb::results::UpdateResult;

type Response<T> = std::result::Result<ApiResponse<T>, ResourceError>;

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
pub async fn create(state: Data<AppState>, body: Json<CreateFilterBody>) -> Response<UpdateResult> {
    // TODO: Validate Calendar ID

    let id = parse_id(&body.user_id)?;
    let filter = mongodb::bson::doc! {
        "_id": id,
        "sources.url": body.url.clone(),
    };

    let new_filter = entity::user::CalendarEventSourceFilters {
        field: body.field.clone(),
        operator: body.operator.clone(),
        value: body.value.clone(),
        calendar_id: body.calendar_id.clone(),
    };

    let update = mongodb::bson::doc! {
        "$push": {
            "sources.$.filters": to_bson(new_filter)
        }
    };

    let result = state
        .db
        .collection::<entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    if result.matched_count == 0 {
        return Err(ResourceError::FailedDatabaseConnection);
    }

    Ok(ApiResponse::from_data(result))
}
