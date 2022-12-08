#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Operation {
    Is = 0,
    IsNot = 1,
    Contains = 2,
    DoesNotContain = 3,
    RegularExpression = 4,
}
