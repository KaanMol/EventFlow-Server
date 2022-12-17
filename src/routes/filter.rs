use crate::routes::{parse_id, to_bson, DataResponse, DebugDataResponse, ErrorResponse, Response};
use crate::{entity, AppState};
use actix_web::dev::Url;
use actix_web::{
    error,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

#[derive(serde::Deserialize, Clone)]
pub struct CreateFilterBody {
    field: String,
    operator: String,
    value: String,
    url: String,
    calendar_id: String,
    user_id: String,
}

#[actix_web::post("/filters")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateFilterBody>,
) -> Result<HttpResponse, Error> {
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

    // FIXME: The inserted filter does not have an unique identifier, so we cannot delete or filter on it later.
    let result = state
        .db
        .collection::<entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not create filter", e)))?;

    if result.matched_count == 0 {
        return Err(error::ErrorBadRequest(ErrorResponse::new(
            "Could not create filter",
            format!(
                "Could not find source with URL '{}' for user with ID '{}'",
                body.url, body.user_id
            ),
        )));
    }

    Ok(HttpResponse::Created().json(DataResponse::new("Created filter", result)))
}
