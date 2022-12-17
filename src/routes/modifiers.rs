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
pub struct CreateModifierBody {
    field: String,
    operation: String,
    value: String,
    new_value: String,
    url: String,
    user_id: String,
}

#[actix_web::post("/modifiers")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateModifierBody>,
) -> Result<HttpResponse, Error> {
    // TODO: Validate Calendar ID

    let id = parse_id(&body.user_id)?;
    let filter = mongodb::bson::doc! {
        "_id": id,
        "sources.url": body.url.clone(),
    };

    let new_filter = entity::user::CalendarEventSourceModifier {
        field: body.field.clone(),
        value: body.value.clone(),
        new_value: body.new_value.clone(),
        operation: body.operation.clone(),
    };

    let update = mongodb::bson::doc! {
        "$push": {
            "sources.$.modifiers": to_bson(new_filter)
        }
    };

    // FIXME: The inserted filter does not have an unique identifier, so we cannot delete or filter on it later.
    let result = state
        .db
        .collection::<entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not create modifier", e)))?;

    if result.matched_count == 0 {
        return Err(error::ErrorBadRequest(ErrorResponse::new(
            "Could not create filter",
            format!(
                "Could not find source with URL '{}' for user with ID '{}'",
                body.url, body.user_id
            ),
        )));
    }

    Ok(HttpResponse::Created().json(DataResponse::new("Created modifier", result)))
}
