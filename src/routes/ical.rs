use crate::routes::reply_not_found;
use crate::routes::ErrorConverter;
use crate::{
    routes::{OptionConverter, ResultConverter},
    AppState,
};

use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use entity::calendar as Calendar;
use entity::ical as Ical;
use entity::user as User;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::ModelTrait;
use sea_orm::{ActiveValue, EntityTrait, QueryFilter, Related};

#[derive(serde::Deserialize, Clone)]
pub struct CreateIcalBody {
    url: String,
    user_id: String
}

#[actix_web::post("/calendar/{calendar_id}/ical")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateIcalBody>,
    calendar_id: Path<String>,
) -> HttpResponse {
    let user: User::Model = match User::Entity::find_by_id(body.user_id.clone())
        .one(&state.database)
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => return reply_not_found("Could not find user"),
        },
        Err(e) => return e.reply(),
    };

    let calendar = match user
        .find_related(Calendar::Entity)
        .filter(Calendar::Column::Id.eq(calendar_id.clone()))
        .one(&state.database)
        .await
    {
        Ok(calendar) => match calendar {
            Some(calendar) => calendar,
            None => return reply_not_found("Could not find calendar"),
        },
        Err(e) => return e.reply(),
    };

    // TODO: Check if the url is valid

    Ical::ActiveModel {
        id: ActiveValue::NotSet,
        calendar: ActiveValue::Set(calendar.id.clone()),
        url: ActiveValue::Set(body.url.clone()),
    }
    .insert(&state.database)
    .await
    .reply()
}

#[actix_web::get("/calendar/{calendar_id}/ical")]
pub async fn read_for_calandar(state: Data<AppState>, calendar_id: Path<String>) -> HttpResponse {
    let calendar = match Calendar::Entity::find_by_id(calendar_id.clone())
        .one(&state.database)
        .await
    {
        Ok(calendar) => match calendar {
            Some(calendar) => calendar,
            None => return reply_not_found("Could not find calendar"),
        },
        Err(e) => return e.reply(),
    };

    calendar
        .find_related(Ical::Entity)
        .all(&state.database)
        .await
        .reply()
}
