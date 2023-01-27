use crate::entity::user::CalendarEventSource;
use crate::handlers::error::ResourceError;
use crate::handlers::response::ApiResponse;
use crate::routes::parse_id;
use crate::{entity, handlers, AppState};
use actix_web::web::{Data, Json, Path};

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
) -> crate::common::Response<CalendarEventSource> {
    // TODO: Validate URL
    let id = parse_id(&body.user_id)?;

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

#[actix_web::get("/sources/{user_id}")]
pub async fn read(
    state: Data<AppState>,
    user_id: Path<String>,
) -> crate::common::Response<Vec<CalendarEventSource>> {
    let id = crate::routes::parse_id(&user_id)?;

    let user = crate::handlers::user::get_user(id, state)
        .await
        .map_err(|_| ResourceError::NotFoundById(id.to_string()))?;

    Ok(ApiResponse::from_data(user.sources))
}
