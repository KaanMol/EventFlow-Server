pub use sea_orm_migration::prelude::*;

mod field;
mod m20220101_000001_create_users;
mod m20221207_121314_create_icals;
mod m20221207_221915_create_calendar;
mod m20221207_223411_create_calendar_filter;
mod m20221207_223420_create_calendar_event_modifier;
mod operation;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users::Migration),
            Box::new(m20221207_221915_create_calendar::Migration),
            Box::new(m20221207_121314_create_icals::Migration),
            Box::new(m20221207_223411_create_calendar_filter::Migration),
            Box::new(m20221207_223420_create_calendar_event_modifier::Migration),
        ]
    }
}
