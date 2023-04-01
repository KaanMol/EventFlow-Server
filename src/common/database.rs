pub async fn mongo_client() -> mongodb::Client {
    let connection_string = std::env::var("DATABASE_URL")
        .expect("Could not find DATABASE_URL in environment variables");

    tracing::info!("Connecting to database: {}", connection_string);

    let client_options = mongodb::options::ClientOptions::parse(connection_string)
        .await
        .unwrap();

    mongodb::Client::with_options(client_options).unwrap()
}

pub async fn connect() -> mongodb::Database {
    mongo_client().await.database("calendarserver")
}
