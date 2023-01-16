pub async fn connect() -> mongodb::Database {
    let client_options =
        mongodb::options::ClientOptions::parse("mongodb://root:example@localhost:27017")
            .await
            .unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();

    client.database("calendarserver")
}
