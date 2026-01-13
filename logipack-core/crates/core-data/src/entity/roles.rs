use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserRoles,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserRoles => Entity::has_many(super::user_roles::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
