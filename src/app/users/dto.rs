use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::app::sources::dto::EventsSourceDto;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub identities: Vec<String>,
    pub sources: Vec<EventsSourceDto>,
}

impl From<crate::entity::user::User> for UserDto {
    fn from(user: crate::entity::user::User) -> Self {
        UserDto {
            id: user.id.unwrap().to_hex(),
            name: user.name,
            identities: user.identities,
            sources: user.sources.into_iter().map(|s| s.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserDto {
    pub name: String,
}
