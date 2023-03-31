use crate::{app, common};

pub mod data;
mod handlers;

pub async fn setup() -> app::State {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::try_init().ok();

    app::State {
        db: common::database::connect_testdb().await,
    }
}
