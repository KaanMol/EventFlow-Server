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
pub async fn create_user(
    state: actix_web::web::Data<super::AppState>,
    user: actix_web::web::Json<CreateUserBody>,
) -> actix_web::HttpResponse {
    reply(super::database::Database::create_user(&state.conn, user.clone()).await)
}

#[actix_web::post("/user/ical")]
pub async fn create_and_link_ical(
    state: actix_web::web::Data<super::AppState>,
    ical: actix_web::web::Json<LinkIcalBody>,
) -> actix_web::HttpResponse {
    reply(super::database::Database::create_and_link_ical(&state.conn, ical.clone()).await)
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
