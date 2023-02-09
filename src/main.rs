mod app;
// mod calendar;
mod common;
mod entity;
pub mod handlers;

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
