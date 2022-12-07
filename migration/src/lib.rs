pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users;
mod m20221207_121314_create_icals;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users::Migration),
            Box::new(m20221207_121314_create_icals::Migration),
        ]
    }
}
