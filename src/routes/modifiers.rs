use crate::handlers::response::ApiResponse;
use crate::routes::parse_id;
use crate::{entity, AppState};
use actix_web::web::{Data, Json};

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
) -> crate::common::Response<entity::user::CalendarEventSourceModifier> {
    // TODO: Validate Calendar ID

    let id = parse_id(&body.user_id)?;

    let new_modifier = entity::user::CalendarEventSourceModifier {
        field: body.field.clone(),
        value: body.value.clone(),
        new_value: body.new_value.clone(),
        operation: body.operation.clone(),
    };

    Ok(ApiResponse::from_data(
        crate::handlers::modifier::create_modifier(id, body.url.clone(), new_modifier, state)
            .await?,
    ))
}
