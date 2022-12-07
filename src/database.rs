use entity::user::Entity as User;
use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::routes::{CreateUserBody, LinkIcalBody};

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

        Ok(Database {
            conn: sea_orm::Database::connect(options).await?,
        })
    }

    pub async fn get_all_users(&self, name: String) -> Vec<entity::user::Model> {
        let user = User::find().all(&self.conn).await.unwrap();
        user
    }

    pub async fn create_user(
        &self,
        user: CreateUserBody,
    ) -> Result<entity::user::Model, sea_orm::DbErr> {
        entity::user::ActiveModel {
            id: sea_orm::ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            name: sea_orm::ActiveValue::Set(user.name),
        }
        .insert(&self.conn)
        .await
    }

    pub async fn create_and_link_ical(
        &self,
        ical: LinkIcalBody,
    ) -> Result<entity::ical::Model, sea_orm::DbErr> {
        entity::ical::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            link: sea_orm::ActiveValue::Set(ical.url),
            user: sea_orm::ActiveValue::Set(ical.user),
        }
        .insert(&self.conn)
        .await
    }

    pub fn get_connection(&self) -> &sea_orm::DatabaseConnection {
        &self.conn
    }
}
