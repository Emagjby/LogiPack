use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "offices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub name: String,
    pub city: String,
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EmployeeOffices,
    Shipments,
    StatusHistory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::EmployeeOffices => Entity::has_many(super::employee_offices::Entity).into(),
            Self::Shipments => Entity::has_many(super::shipments::Entity).into(),
            Self::StatusHistory => {
                Entity::has_many(super::shipment_status_history::Entity).into()
            }
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
