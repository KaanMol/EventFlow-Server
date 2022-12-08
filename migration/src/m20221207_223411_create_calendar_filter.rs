use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CalendarFilter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CalendarFilter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CalendarFilter::Calendar).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("calendarId")
                            .from(CalendarFilter::Table, CalendarFilter::Calendar)
                            .to(
                                crate::m20221207_221915_create_calendar::Calendar::Table,
                                crate::m20221207_221915_create_calendar::Calendar::Id,
                            ),
                    )
                    .col(
                        ColumnDef::new(Calendar::Field)
                            .enumeration("field", super::field::Field)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Calendar::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(CalendarFilter::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CalendarFilter {
    Table,
    Id,
    Calendar,
    Field,
    Operation,
    Value,
}
