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
pub struct CreateSourceBody {
    name: String,
    url: String,
    user_id: String,
}

#[actix_web::post("/sources")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateSourceBody>,
) -> Result<HttpResponse, Error> {
    // TODO: Validate URL

    let id = parse_id(&body.user_id)?;
    let filter = mongodb::bson::doc! {
        "_id": id
    };

    let new_source = entity::user::CalendarEventSource {
        name: body.name.clone(),
        url: body.url.clone(),
        filters: vec![],
        modifiers: vec![],
    };

    let update = mongodb::bson::doc! {
        "$push": {
            "sources": to_bson(new_source)
        }
    };

    // FIXME: The inserted source does not have an unique identifier, so we cannot delete it later.
    //        We need to add an unique identifier to the source. Perhaps the URL?
    let result = state
        .db
        .collection::<entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not create source", e)))?;

    Ok(HttpResponse::Created().json(DataResponse::new("Created source", result)))
}

#[actix_web::get("/sources/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> Result<HttpResponse, Error> {
    // TODO: Validate URL

    let id = crate::routes::parse_id(user_id.to_string())?;

    let user = state
        .db
        .collection::<entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "_id": id
            },
            None,
        )
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not query users", e)))?
        .ok_or_else(|| {
            error::ErrorNotFound(Response::new(format!(
                "Could not find user with id {}",
                user_id
            )))
        })?;

    Ok(HttpResponse::Ok().json(DataResponse::new("Found user", user.sources)))
}
