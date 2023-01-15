use crate::{
    entity::{self, user},
    handlers::error::ResourceError,
    routes::{DataResponse, ErrorResponse, Response},
    AppState,
};
use actix_web::{
    error,
    web::{Data, Json, Path},
    Error, HttpResponse, Responder,
};

#[derive(serde::Deserialize, Clone)]
pub struct CreateUserBody {
    name: String,
}

#[actix_web::post("/users")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Result<HttpResponse, Error> {
    let result = state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(
            entity::user::User {
                name: body.name.clone(),
                sources: vec![],
            },
            None,
        )
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not create user", e)))?;

    // let user = state.db.collection::<entity::user::User>("users").g
    Ok(HttpResponse::Created().json(DataResponse::new("Created user", result.inserted_id)))
}

#[actix_web::get("/users/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> Result<HttpResponse, Error> {
    let id = crate::routes::parse_id(user_id.to_string())?;

    let user = crate::handlers::user::get_user(id, state).await;
    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(DataResponse::new("Found user", user))),
        Err(error) => Ok(error_handler(error)),
    }
}

#[derive(serde::Serialize)]
struct ApiResponse<T> {
    success: bool,
    error: Option<ResponseError>,
    data: Option<T>,
}
#[derive(serde::Serialize)]
struct ResponseError {
    code: i32,
    message: String,
}

pub fn error_handler(error: crate::handlers::error::ResourceError) -> HttpResponse {
    match error {
        ResourceError::NotFoundById(id) => HttpResponse::NotFound().json(ApiResponse::<String> {
            success: false,
            data: None,
            error: Some(ResponseError {
                code: 404,
                message: String::from("Requested resource not found"),
            }),
        }),
        _ => HttpResponse::Gone().json(ApiResponse::<String> {
            success: false,
            data: None,
            error: Some(ResponseError {
                code: 404,
                message: String::from("idk bruh"),
            }),
        }),
    }
}

// // TODO: Update user

// // TODO: Delete user
