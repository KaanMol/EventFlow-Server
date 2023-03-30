pub async fn connect() -> mongodb::Database {
    let connection_string = std::env::var("DATABASE_URL")
        .expect("Could not find DATABASE_URL in environment variables");

    tracing::info!("Connecting to database: {}", connection_string);

    let client_options = mongodb::options::ClientOptions::parse(connection_string)
        .await
        .unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();

    client.database("calendarserver")
}
