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
                    .table(Calendar::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Calendar::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Calendar::Name).string().not_null())
                    .col(ColumnDef::new(Calendar::User).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-calendar-user_id")
                            .from(Calendar::Table, Calendar::User)
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
            .drop_table(Table::drop().table(Calendar::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Calendar {
    Table,
    Id,
    Name,
    User,
}
