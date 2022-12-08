pub struct Database;

impl Database {
    pub async fn connect() -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
        let mut options = sea_orm::ConnectOptions::new("sqlite://db.sqlite?mode=rwc".to_owned());

        options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(std::time::Duration::from_secs(8))
            .acquire_timeout(std::time::Duration::from_secs(8))
            .idle_timeout(std::time::Duration::from_secs(8))
            .max_lifetime(std::time::Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(tracing::log::LevelFilter::Info);

        sea_orm::Database::connect(options).await
    }
}
