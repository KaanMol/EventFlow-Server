use crate::handlers::response::ApiResponse;
use crate::UserClaims;
use crate::{
    common::SecurityAddon,
    dto::{
        calendar::{EventSourceDto, EventSourceFilterDto, EventSourceModifierDto},
        user::UserDto,
    },
};
use crate::{
    entity::{self, user::User},
    handlers::error::ResourceError,
    AppState,
};
use actix_web::web::{Data, Json, ReqData};
use actix_web::{dev::ServiceFactory, get};
use utoipa::OpenApi;

// // TODO: Update user

// // TODO: Delete user
