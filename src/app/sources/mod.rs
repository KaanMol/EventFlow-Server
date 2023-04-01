use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
};
use actix_web_httpauth::middleware::HttpAuthentication;
use utoipa::OpenApi;

use self::dto::EventsSourceDto;

pub mod dto;
pub mod routes;

#[derive(OpenApi)]
#[openapi(
        paths(
    		// read,
			// create
        ),
		components(
			schemas(
				EventsSourceDto
			)
		),
        tags(
            (name = "Sources", description = "Event sources management endpoint")
        ),
		// modifiers(&SecurityAddon)
	)]
pub struct ApiDoc;

pub fn routes() -> actix_web::Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    // Initialise the JWT validator middleware
    let auth = HttpAuthentication::bearer(super::middleware::auth_validator);

    actix_web::web::scope("/sources")
        .wrap(auth)
        .service(routes::create)
        .service(routes::sync)
}
