async fn mongo_client() -> mongodb::Client {
    let connection_string = std::env::var("DATABASE_URL")
        .expect("Could not find DATABASE_URL in environment variables");

    tracing::info!("Connecting to database: {}", connection_string);

    let client_options = mongodb::options::ClientOptions::parse(connection_string)
        .await
        .unwrap();

    let client = mongodb::Client::with_options(client_options).unwrap();

    // Check if client is connected
    client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 }, None)
        .await
        .expect("Could not connect to database");

    client
}

pub async fn connect() -> mongodb::Database {
    mongo_client().await.database("calendarserver")
}

#[allow(dead_code)] // This is not actually dead code, but clippy doesn't realise that tests are entry points.
pub async fn connect_testdb() -> mongodb::Database {
    let client = mongo_client().await;

    client.database("testdb").drop(None).await.unwrap();
    client.database("testdb")
}
