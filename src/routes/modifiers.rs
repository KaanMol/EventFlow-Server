use crate::handlers::response::ApiResponse;
use crate::{entity, AppState, UserClaims};
use actix_web::web::{Data, Json, ReqData};

#[derive(serde::Deserialize, Clone)]
pub struct CreateModifierBody {
    field: String,
    operation: String,
    value: String,
    new_value: String,
    url: String,
}

#[actix_web::post("")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateModifierBody>,
    user_claims: ReqData<UserClaims>,
) -> crate::common::Response<entity::user::CalendarEventSourceModifier> {
    // TODO: Validate Calendar ID

    let user_identity = user_claims.into_inner().cid;

    let new_modifier = entity::user::CalendarEventSourceModifier {
        field: body.field.clone(),
        value: body.value.clone(),
        new_value: body.new_value.clone(),
        operation: body.operation.clone(),
    };

    Ok(ApiResponse::from_data(
        crate::handlers::modifier::create_modifier(
            user_identity,
            body.url.clone(),
            new_modifier,
            state,
        )
        .await?,
    ))
}
