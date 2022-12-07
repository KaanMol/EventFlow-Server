#[derive(serde::Deserialize)]
pub struct CreateUserBody {
    name: String,
}

#[actix_web::post("/user")]
pub async fn create_user(
    state: actix_web::web::Data<super::AppState>,
    user: actix_web::web::Json<CreateUserBody>,
) -> actix_web::HttpResponse {
    reply(super::database::Database::create_user(&state.conn, user.name.clone()).await)
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
