use utoipa::OpenApi;

use self::dto::EventRuleDto;

pub mod dto;

#[derive(OpenApi)]
#[openapi(
        paths(
    		// read,
			// create
        ),
		components(
			schemas(
				EventRuleDto
			)
		),
        tags(
            (name = "EventRules", description = "Event rules management endpoint")
        ),
		// modifiers(&SecurityAddon)
	)]
pub struct ApiDoc;
