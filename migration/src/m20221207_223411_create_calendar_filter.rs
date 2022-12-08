use sea_orm::IdenStatic;
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
                        ColumnDef::new(CalendarFilter::Field)
                            .enumeration(
                                super::field::Field::Enum,
                                [super::field::Field::Title, super::field::Field::Description],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CalendarFilter::Operation)
                            .enumeration(
                                super::operation::Operation::Enum,
                                [
                                    super::operation::Operation::Is,
                                    super::operation::Operation::IsNot,
                                    super::operation::Operation::Contains,
                                    super::operation::Operation::DoesNotContain,
                                    super::operation::Operation::RegularExpression,
                                ],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(CalendarFilter::Value).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
