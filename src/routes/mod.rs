pub mod ical;
pub mod user;

trait HttpResponseConverter<E: std::error::Error> {
    fn reply(self) -> actix_web::HttpResponse;
}

// Convert a Result<T, E> into a HttpResponse
impl<T: serde::Serialize, E: std::error::Error> HttpResponseConverter<E> for Result<T, E> {
    fn reply(self) -> actix_web::HttpResponse {
        match self {
            Ok(result) => actix_web::HttpResponse::Ok().json(result),
            Err(e) => e.reply(),
        }
    }
}

// Convert an Error into a HttpResponse
impl<E: std::error::Error> HttpResponseConverter<E> for E {
    fn reply(self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().json(format!("{:#?}", self))
    }
}
