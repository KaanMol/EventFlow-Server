use actix_web::HttpResponse;

pub mod calendar;
pub mod user;
pub mod ical;

trait Converter {
    fn reply(self) -> HttpResponse;
}

trait ResultConverter {
    fn reply(self) -> HttpResponse;
}

trait OptionConverter {
    fn reply_option(self, message: impl Into<String>) -> HttpResponse;
}

trait ErrorConverter<E: std::error::Error> {
    fn reply(self) -> HttpResponse;
}

impl<T: serde::Serialize> Converter for T {
    fn reply(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl<T: serde::Serialize, E: std::error::Error> ResultConverter for Result<T, E> {
    fn reply(self) -> HttpResponse {
        match self {
            Ok(result) => result.reply(),
            Err(e) => e.reply(),
        }
    }
}

impl<T: serde::Serialize, E: std::error::Error> OptionConverter for Result<Option<T>, E> {
    fn reply_option(self, message: impl Into<String>) -> HttpResponse {
        match self {
            Ok(result) => match result {
                Some(result) => result.reply(),
                None => reply_not_found(message),
            },
            Err(e) => e.reply(),
        }
    }
}

impl<E: std::error::Error> ErrorConverter<E> for E {
    fn reply(self) -> HttpResponse {
        HttpResponse::InternalServerError().json(format!("{:?}", self))
    }
}

pub struct Response {
    pub status: reqwest::StatusCode,
    pub data: String,
}

pub fn reply_not_found(message: impl Into<String>) -> HttpResponse {
    HttpResponse::NotFound().json(format!("Not found: {}", message.into()))
}
