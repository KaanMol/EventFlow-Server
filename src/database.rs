use entity::user::Entity as User;
use sea_orm::{ActiveModelTrait, EntityTrait};
// struct User {
//     id: String,
//     name: String,
// }

pub struct Ical {
    pub id: Option<i32>,
    pub user_id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Database {
    conn: sea_orm::DatabaseConnection,
}

impl Database {
    pub async fn connect() -> Result<Self, sea_orm::DbErr> {
        // let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut options = sea_orm::ConnectOptions::new("sqlite://users.sqlite?mode=rwc".to_owned());

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

    pub async fn get_all_users(conn: &sea_orm::DbConn, name: String) -> Vec<entity::user::Model> {
        let user = User::find().all(conn).await.unwrap();
        user
    }

    pub async fn create_user(
        conn: &sea_orm::DbConn,
        name: String,
    ) -> Result<entity::user::Model, sea_orm::DbErr> {
        entity::user::ActiveModel {
            id: sea_orm::ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            name: sea_orm::ActiveValue::Set(name),
        }
        .insert(conn)
        .await
    }

    pub fn get_connection(&self) -> &sea_orm::DatabaseConnection {
        &self.conn
    }
}
