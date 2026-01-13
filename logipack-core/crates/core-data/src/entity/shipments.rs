use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "shipments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub client_id: Uuid,

    pub current_status: String,

    pub current_office_id: Option<Uuid>,

    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Client,
    CurrentOffice,
    StatusHistory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Client => Entity::belongs_to(super::clients::Entity)
                .from(Column::ClientId)
                .to(super::clients::Column::Id)
                .into(),
            Self::CurrentOffice => Entity::belongs_to(super::offices::Entity)
                .from(Column::CurrentOfficeId)
                .to(super::offices::Column::Id)
                .into(),
            Self::StatusHistory => Entity::has_many(super::shipment_status_history::Entity).into(),
        }
    }
}

impl Related<super::clients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Client.def()
    }
}

impl Related<super::offices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurrentOffice.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
