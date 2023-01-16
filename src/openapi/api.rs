use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
        paths(
            crate::routes::user::read,
        ),
        components(
            schemas(crate::entity::user::User, crate::entity::user::CalendarEventSource, crate::entity::user::CalendarEventSourceFilters, crate::entity::user::CalendarEventSourceModifier)
        ),
        tags(
            (name = "eventflow", description = "The new calendar API."),
        ),
        // modifiers(&SecurityAddon)
    )]

pub struct ApiDoc;
