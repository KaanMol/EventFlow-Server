use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
};
use actix_web_httpauth::middleware::HttpAuthentication;
use utoipa::OpenApi;

use self::{dto::EventsSourceDto, routes::create};

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
        Response = ServiceResponse<BoxBody>, // FIXME: change to EitherBody<BoxBody>
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    // Initialise the JWT validator middleware
    let auth = HttpAuthentication::bearer(super::middleware::auth_validator);

    // FIXME: add auth back
    actix_web::web::scope("/sources")
        .service(routes::create)
        .service(routes::sync)
}
