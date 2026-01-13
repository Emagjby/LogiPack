use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "shipment_status_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub shipment_id: Uuid,

    pub from_status: Option<String>,
    pub to_status: String,

    pub changed_at: DateTimeWithTimeZone,

    pub actor_user_id: Option<Uuid>,
    pub office_id: Option<Uuid>,

    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Shipment,
    ActorUser,
    Office,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Shipment => Entity::belongs_to(super::shipments::Entity)
                .from(Column::ShipmentId)
                .to(super::shipments::Column::Id)
                .into(),
            Self::ActorUser => Entity::belongs_to(super::users::Entity)
                .from(Column::ActorUserId)
                .to(super::users::Column::Id)
                .into(),
            Self::Office => Entity::belongs_to(super::offices::Entity)
                .from(Column::OfficeId)
                .to(super::offices::Column::Id)
                .into(),
        }
    }
}

impl Related<super::shipments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Shipment.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActorUser.def()
    }
}

impl Related<super::offices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Office.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
