use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "streams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub head_hash: Option<Vec<u8>>,

    pub kind: String,

    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Packages,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Packages => Entity::has_many(super::packages::Entity).into(),
        }
    }
}

impl Related<super::packages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Packages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
