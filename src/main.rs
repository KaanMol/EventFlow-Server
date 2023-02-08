mod app;
// mod calendar;
mod common;
// pub mod dto;
mod entity;
pub mod handlers;
// mod routes;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    app::start().await
}

fn test() {}
