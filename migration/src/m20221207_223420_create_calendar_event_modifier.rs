use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CalendarEventModifier::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CalendarEventModifier::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CalendarEventModifier::Calendar)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("calendarId")
                            .from(
                                CalendarEventModifier::Table,
                                CalendarEventModifier::Calendar,
                            )
                            .to(
                                crate::m20221207_221915_create_calendar::Calendar::Table,
                                crate::m20221207_221915_create_calendar::Calendar::Id,
                            ),
                    )
                    .col(
                        ColumnDef::new(CalendarEventModifier::Field)
                            .enumeration(
                                super::field::Field::Enum,
                                [super::field::Field::Title, super::field::Field::Description],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CalendarEventModifier::Operation)
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
                    .col(
                        ColumnDef::new(CalendarEventModifier::Value)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CalendarEventModifier::NewValue)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CalendarEventModifier::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CalendarEventModifier {
    Table,
    Id,
    Calendar,
    Field,
    Operation,
    Value,
    NewValue,
}
