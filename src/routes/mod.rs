pub mod ical;
pub mod user;

trait Converter {
    fn reply(self) -> actix_web::HttpResponse;
}

trait ResultConverter {
    fn reply(self) -> actix_web::HttpResponse;
}

trait OptionConverter {
    fn reply_option(self) -> actix_web::HttpResponse;
}

trait ErrorConverter<E: std::error::Error> {
    fn reply(self) -> actix_web::HttpResponse;
}

impl<T: serde::Serialize> Converter for T {
    fn reply(self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl<T: serde::Serialize, E: std::error::Error> ResultConverter for Result<T, E> {
    fn reply(self) -> actix_web::HttpResponse {
        match self {
            Ok(result) => result.reply(),
            Err(e) => e.reply(),
        }
    }
}

impl<T: serde::Serialize, E: std::error::Error> OptionConverter for Result<Option<T>, E> {
    fn reply_option(self) -> actix_web::HttpResponse {
        match self {
            Ok(result) => match result {
                Some(result) => result.reply(),
                None => actix_web::HttpResponse::NotFound().json("No entities found"),
            },
            Err(e) => e.reply(),
        }
    }
}

impl<E: std::error::Error> ErrorConverter<E> for E {
    fn reply(self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().json(format!("{:?}", self))
    }
}
