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
                    .col(ColumnDef::new(Ical::Link).string().not_null())
                    .col(ColumnDef::new(Ical::User).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_ical")
                            .from(Ical::Table, Ical::User)
                            .to(
                                crate::m20220101_000001_create_users::User::Table,
                                crate::m20220101_000001_create_users::User::Id,
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
pub enum Ical {
    Table,
    Id,
    Link,
    User,
}
