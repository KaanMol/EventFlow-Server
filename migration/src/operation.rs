use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Operation {
    #[iden = "operation"]
    Enum,
    #[iden = "Is"]
    Is,
    #[iden = "IsNot"]
    IsNot,
    #[iden = "Contains"]
    Contains,
    #[iden = "DoesNotContain"]
    DoesNotContain,
    #[iden = "RegularExpression"]
    RegularExpression,
}
