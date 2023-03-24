use actix_web::{get, Responder};

#[utoipa::path(
	tag = "Calendar Application",
    responses(
        (status = 200, description = "pong", body = [String])
    )
)]
#[get("/ping")]
pub async fn ping() -> impl Responder {
    "pong"
}
