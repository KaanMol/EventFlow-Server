mod dto;
pub mod routes;

use crate::app::users::routes::{ __path_create, create, __path_read, read };
use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, body::{BoxBody, EitherBody}};
use actix_web_httpauth::middleware::HttpAuthentication;
use dto::{CreateUserDto, UserDto};
use utoipa::OpenApi;
use crate::common::SecurityAddon;

use self::routes::update;

#[derive(OpenApi)]
#[openapi(
        paths(
    		read,
			create
        ),
		components(
			schemas(
				UserDto, 
				CreateUserDto
			)
		),
        tags(
            (name = "Users", description = "Users management endpoint")
        ),
		modifiers(&SecurityAddon)
	)]
pub struct ApiDoc;

pub fn routes() -> actix_web::Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<EitherBody<BoxBody>>, Error = actix_web::Error, InitError = ()>> {
	// Initialise the JWT validator middleware
	let auth = HttpAuthentication::bearer(super::middleware::auth_validator);

	actix_web::web::scope("/users")
		.wrap(auth)
		.service(read)
		.service(create)
		.service(update)
}