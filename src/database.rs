struct User {
    id: String,
    name: String,
}

pub struct Ical {
    pub id: Option<i32>,
    pub user_id: String,
    pub name: String,
    pub url: String,
}

pub struct Database {
    conn: sea_orm::DatabaseConnection,
}

impl Database {
    pub async fn connect(url: String) -> Result<Self, sea_orm::DbErr> {
        let mut options = sea_orm::ConnectOptions::new(url);

        options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(std::time::Duration::from_secs(8))
            .acquire_timeout(std::time::Duration::from_secs(8))
            .idle_timeout(std::time::Duration::from_secs(8))
            .max_lifetime(std::time::Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(tracing::log::LevelFilter::Info);

        Ok(Database {
            conn: sea_orm::Database::connect(options).await?,
        })
    }
}
