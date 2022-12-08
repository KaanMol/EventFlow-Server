use crate::{routes::HttpResponseConverter, AppState};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use entity::user as User;
use sea_orm::{EntityTrait, QueryFilter, Related};

#[actix_web::get("/user/{user_id}")]
pub async fn get_icals_for_user(state: Data<AppState>, path: Path<String>) -> HttpResponse {
    // Find the user from the database
    let user = match User::Entity::find_by_id(path.to_string())
        .one(&state.database)
        .await
    {
        Ok(user) => user,
        Err(e) => return e.reply(),
    };

    // Check if the user exists
    let user = match user {
        Some(user) => user,
        None => return HttpResponse::NotFound().json("User not found"),
    };

    // Find all the icals for the user
    User::Entity::find_related()
        .belongs_to(&user)
        .all(&state.database)
        .await
        .reply()
}
