// use crate::{
//     entities::{self, user},
//     AppState,
// };

// use super::error::ResourceError;

// pub async fn create_source(
//     user_identity: String,
//     new_source: user::CalendarEventSource,
//     state: actix_web::web::Data<AppState>,
// ) -> Result<user::CalendarEventSource, super::error::ResourceError> {
//     let filter = mongodb::bson::doc! {
//         "identities": {
//             "$elemMatch": {
//                 "$in": [user_identity]
//             }
//         }
//     };

//     let update = mongodb::bson::doc! {
//         "$push": {
//             "sources": super::to_bson(&new_source)
//         }
//     };

//     state
//         .db
//         .collection::<entities::user::User>("users")
//         .update_one(filter, update, None)
//         .await
//         .map_err(|_| ResourceError::FailedDatabaseConnection)?;

//     Ok(new_source)
// }
