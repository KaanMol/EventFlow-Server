use utoipa::OpenApi;

use crate::common::SecurityAddon;

use self::dto::EventFilterDto;

pub mod dto;

#[derive(OpenApi)]
#[openapi(
        paths(
    		// read,
			// create
        ),
		components(
			schemas(
				EventFilterDto
			)
		),
        tags(
            (name = "EventFilters", description = "Event Filters management endpoint")
        ),
		// modifiers(&SecurityAddon)
	)]
pub struct ApiDoc;
