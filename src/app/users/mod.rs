mod dto;
pub mod routes;

use crate::app::users::routes::{ __path_create, create, __path_read, read };
use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, body::{EitherBody, BoxBody}};
use actix_web_httpauth::middleware::HttpAuthentication;
use dto::{ CreateUserDto, UserDto };
use super::{event::dto::{ EventSourceDto, EventSourceFilterDto, EventSourceModifierDto }};
use utoipa::OpenApi;
use crate::common::SecurityAddon;

#[derive(OpenApi)]
#[openapi(
        paths(
    		read,
			create
        ),
		components(
			schemas(
				UserDto, 
				CreateUserDto, 
				EventSourceDto,
				EventSourceFilterDto,
				EventSourceModifierDto
			)
		),
        tags(
            (name = "Users", description = "Users management endpoint")
        ),
		modifiers(&SecurityAddon)
	)]
pub struct ApiDoc;

pub fn test() -> actix_web::Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<EitherBody<BoxBody>>, Error = actix_web::Error, InitError = ()>> {
	// Initialise the JWT validator middleware
	let auth = HttpAuthentication::bearer(super::middleware::auth_validator);

	actix_web::web::scope("/users")
	.wrap(auth)
	.service(read)
	// .service(routes::user::read)
}