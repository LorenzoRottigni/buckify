use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub path: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub public: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl Related<Relation> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
