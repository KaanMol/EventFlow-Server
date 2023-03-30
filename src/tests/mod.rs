use crate::{app, common};

pub mod data;
mod handlers;

pub async fn setup() -> app::State {
    //dotenv::dotenv().ok();

    app::State {
        db: common::database::connect_testdb().await,
    }
}
