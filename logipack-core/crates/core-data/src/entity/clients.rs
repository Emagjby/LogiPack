use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "clients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Shipments,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Shipments => Entity::has_many(super::shipments::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
