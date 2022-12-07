use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    pub name: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct LinkIcalBody {
    pub user: String,
    pub url: String,
}

#[actix_web::post("/user")]
pub async fn create_user(state: Data<super::AppState>, body: Json<CreateUserBody>) -> HttpResponse {
    reply(state.database.create_user(body.clone()).await)
}

#[actix_web::post("/user/ical")]
pub async fn create_and_link_ical(
    state: Data<super::AppState>,
    body: Json<LinkIcalBody>,
) -> HttpResponse {
    reply(state.database.create_and_link_ical(body.clone()).await)
}

#[actix_web::get("/user/{user_id}")]
pub async fn get_icals_for_user(
    state: Data<super::AppState>,
    user_id: Path<String>,
) -> HttpResponse {
    reply(state.database.get_icals_for_user(user_id.clone()).await)
}

pub fn reply<T: serde::Serialize>(
    result: Result<T, impl std::error::Error>,
) -> actix_web::HttpResponse {
    match result {
        Ok(result) => actix_web::HttpResponse::Ok().json(result),
        Err(e) => actix_web::HttpResponse::InternalServerError().json(WebError {
            message: e.to_string(),
        }),
    }
}

#[derive(serde::Serialize)]
struct WebError {
    message: String,
}
