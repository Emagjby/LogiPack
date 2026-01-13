use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "packages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub hash: Vec<u8>,

    pub stream_id: Uuid,

    pub prev_hash: Option<Vec<u8>>,

    pub scb: Vec<u8>,

    pub created_at: DateTimeWithTimeZone,

    pub event_type: String,

    pub seq: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Stream,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Stream => Entity::belongs_to(super::streams::Entity)
                .from(Column::StreamId)
                .to(super::streams::Column::Id)
                .into(),
        }
    }
}

impl Related<super::streams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stream.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
