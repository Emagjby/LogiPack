use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub email: Option<String>,

    pub password_hash: Option<String>,

    pub auth0_sub: Option<String>,

    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Employee,
    StatusHistory,
    UserRoles,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Employee => Entity::has_one(super::employees::Entity).into(),
            Self::StatusHistory => Entity::has_many(super::shipment_status_history::Entity).into(),
            Self::UserRoles => Entity::has_many(super::user_roles::Entity).into(),
        }
    }
}

impl Related<super::shipment_status_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StatusHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
