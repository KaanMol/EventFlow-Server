use crate::entity::user::CalendarEventSource;
use crate::handlers::error::ResourceError;
use crate::handlers::response::ApiResponse;
use crate::routes::{parse_id, to_bson};
use crate::{entity, AppState};
use actix_web::web::{Data, Json, Path};
use mongodb::results::UpdateResult;

type Response<T> = std::result::Result<ApiResponse<T>, ResourceError>;

#[derive(serde::Deserialize, Clone)]
pub struct CreateSourceBody {
    name: String,
    url: String,
    user_id: String,
}

// FIXME: Proper return type instead of UpdateResult
#[actix_web::post("/sources")]
pub async fn create(state: Data<AppState>, body: Json<CreateSourceBody>) -> Response<UpdateResult> {
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

    let result = state
        .db
        .collection::<entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    Ok(ApiResponse::from_data(result))
}

#[actix_web::get("/sources/{user_id}")]
pub async fn read(
    state: Data<AppState>,
    user_id: Path<String>,
) -> Response<Vec<CalendarEventSource>> {
    // TODO: Validate URL

    let id = crate::routes::parse_id(&user_id)?;

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
        .map_err(|_| ResourceError::FailedDatabaseConnection)?
        .ok_or_else(|| ResourceError::NotFoundById(id.to_string()))?;

    Ok(ApiResponse::from_data(user.sources))
}
