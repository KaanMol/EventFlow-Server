use crate::AppState;
use actix_web::{
    error,
    web::{Data, Json, Query},
    Error, HttpResponse,
};
use entity::calendar as Calendar;
use entity::user as User;
use sea_orm::ActiveModelTrait;
use sea_orm::ModelTrait;
use sea_orm::{ActiveValue, EntityTrait};
use uuid::Uuid;

#[derive(serde::Deserialize, Clone)]
pub struct CreateCalendarBody {
    name: String,
    user_id: String,
}

#[actix_web::post("/calendars")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateCalendarBody>,
) -> Result<HttpResponse, Error> {
    // FIXME: The same algorithm is used in multiple places to find a user by id.
    //        This should be refactored into a function.
    let user = User::Entity::find_by_id(body.user_id.clone())
        .one(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?
        .ok_or_else(|| {
            error::ErrorNotFound(format!("Could not find user with id {}", body.user_id))
        })?;

    let calendar = Calendar::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4().to_string()),
        user: ActiveValue::Set(user.id.clone()),
        name: ActiveValue::Set(body.name.clone()),
    }
    .insert(&state.database)
    .await
    .map_err(|_| error::ErrorBadRequest("Could not create calendar"))?;

    Ok(HttpResponse::Created().json(calendar))
}

#[derive(serde::Deserialize, Clone)]
pub struct FindCalendarQuery {
    user: String,
}

#[actix_web::get("/calendars")]
pub async fn read_for_user(
    query: Query<FindCalendarQuery>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    // FIXME: The same algorithm is used in multiple places to find a user by id.
    //        This should be refactored into a function.
    let user: User::Model = User::Entity::find_by_id(query.user.clone())
        .one(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?
        .ok_or_else(|| {
            error::ErrorNotFound(format!("Could not find user with id {}", query.user))
        })?;

    let calendars = user
        .find_related(Calendar::Entity)
        .all(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query calendars"))?;

    Ok(HttpResponse::Ok().json(calendars))
}

// TODO: Update calendar

// TODO: Delete calendar
