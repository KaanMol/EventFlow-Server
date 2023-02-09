use super::{
    filters::dto::EventFilterDto,
    rules::dto::EventRuleDto,
    sources::dto::EventsSourceDto,
    users::routes::{__path_create, __path_read, create, read},
};
use crate::common::SecurityAddon;
use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
};
use actix_web_httpauth::middleware::HttpAuthentication;
use utoipa::OpenApi;

pub mod dto;
pub mod routes;

#[derive(OpenApi)]
#[openapi(
        paths(
    		read,
			create
        ),
		components(
			schemas(
			)
		),
        tags(
            (name = "Events", description = "Events management endpoint")
        ),
		// modifiers(&SecurityAddon)
	)]
pub struct ApiDoc;

pub fn routes() -> actix_web::Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    // Initialise the JWT validator middleware
    let auth = HttpAuthentication::bearer(super::middleware::auth_validator);

    actix_web::web::scope("/events")
        // .wrap(auth)
        .service(read)
        .service(create)
}
