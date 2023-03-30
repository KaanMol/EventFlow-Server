use crate::{app, common};

pub mod data;
mod events;
mod users;

pub async fn setup() -> app::State {
    //dotenv::dotenv().ok();

    app::State {
        db: common::database::connect().await,
    }
}
