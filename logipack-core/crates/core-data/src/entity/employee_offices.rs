use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "employee_offices")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub employee_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub office_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Employee,
    Office,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Employee => Entity::belongs_to(super::employees::Entity)
                .from(Column::EmployeeId)
                .to(super::employees::Column::Id)
                .into(),
            Self::Office => Entity::belongs_to(super::offices::Entity)
                .from(Column::OfficeId)
                .to(super::offices::Column::Id)
                .into(),
        }
    }
}

impl Related<super::employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<super::offices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Office.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
