use crate::app::routes::__path_ping;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
		ping
    ),
    tags(
        (name = "Calendar Application", description = "General API endpoints for the calendar application")
    )
)]
pub struct RootApiDoc;
