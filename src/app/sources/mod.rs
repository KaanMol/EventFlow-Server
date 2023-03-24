use utoipa::OpenApi;

use self::dto::EventsSourceDto;

pub mod dto;

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
