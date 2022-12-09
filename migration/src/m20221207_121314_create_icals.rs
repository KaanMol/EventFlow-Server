use sea_orm_migration::prelude::*;

// Disable warnings for unreachable code

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ical::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ical::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ical::Url).string().not_null())
                    .col(ColumnDef::new(Ical::Calendar).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ical-user_id")
                            .from(Ical::Table, Ical::Calendar)
                            .to(
                                crate::m20221207_221915_create_calendar::Calendar::Table,
                                crate::m20221207_221915_create_calendar::Calendar::Id,
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ical::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Ical {
    Table,
    Id,
    Url,
    Calendar,
}
