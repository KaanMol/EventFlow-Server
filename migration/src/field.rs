use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Field {
    #[iden = "field"]
    Enum,
    #[iden = "Title"]
    Title,
    #[iden = "Description"]
    Description,
}
