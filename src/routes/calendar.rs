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
use entity::user as User;
use sea_orm::ActiveModelTrait;
use sea_orm::ModelTrait;
use sea_orm::{ActiveValue, EntityTrait, QueryFilter, Related};

#[derive(serde::Deserialize, Clone)]
pub struct CreateCalendarBody {
    name: String,
    user_id: String,
}

#[actix_web::post("/calendar")]
pub async fn create(state: Data<AppState>, body: Json<CreateCalendarBody>) -> HttpResponse {
    let user = match User::Entity::find_by_id(body.user_id.clone())
        .one(&state.database)
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => return reply_not_found("Could not find user"),
        },
        Err(e) => return e.reply(),
    };

    Calendar::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
        user: ActiveValue::Set(user.id.clone()),
        name: ActiveValue::Set(body.name.clone()),
    }
    .insert(&state.database)
    .await
    .reply()
}

#[actix_web::get("/calendar")]
pub async fn read_for_user(state: Data<AppState>) -> HttpResponse {
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

    user.find_related(Calendar::Entity)
        .all(&state.database)
        .await
        .reply()
}

// TODO: Delte calendar
