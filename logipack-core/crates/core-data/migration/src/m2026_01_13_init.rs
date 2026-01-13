use sea_orm_migration::prelude::*;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Enable uuid generation extension for PostgreSQL
        manager
            .create_extension(Extension::create()
                .name("pgcrypto")
                .to_owned())
            .await()
            .ok();

        // Users
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Roles
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Roles::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        // User Roles M:N
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoles::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserRoles::RoleId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(UserRoles::UserId)
                            .col(UserRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_user")
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_role")
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Employees
        manager
            .create_table(
                Table::create()
                    .table(Employees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Employees::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Employees::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Employees::FullName)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_employees_user")
                            .from(Employees::Table, Employees::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // Offices
        manager
            .create_table(
                Table::create()
                    .table(Offices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Offices::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Offices::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Offices::City)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Offices::Address)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Employee Offices M:N
        manager
            .create_table(
                Table::create()
                    .table(EmployeeOffices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmployeeOffices::EmployeeId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmployeeOffices::OfficeId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(EmployeeOffices::EmployeeId)
                            .col(EmployeeOffices::OfficeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_employee_offices_employee")
                            .from(EmployeeOffices::Table, EmployeeOffices::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_employee_offices_office")
                            .from(EmployeeOffices::Table, EmployeeOffices::OfficeId)
                            .to(Offices::Table, Offices::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Clients
        manager
            .create_table(
                Table::create()
                    .table(Clients::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Clients::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Clients::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Clients::Phone)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Clients::Email)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;
                
        // Shipments
        manager.create_table(
            Table::create()
                .table(Shipments::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Shipments::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(Shipments::ClientId)
                        .uuid()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Shipments::CurrentStatus)
                        .string()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Shipments::CurrentOfficeId)
                        .uuid()
                        .null(),
                )
                .col(
                    ColumnDef::new(Shipments::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(Shipments::UpdatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_shipments_client")
                        .from(Shipments::Table, Shipments::ClientId)
                        .to(Clients::Table, Clients::Id)
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_shipments_current_office")
                        .from(Shipments::Table, Shipments::CurrentOfficeId)
                        .to(Offices::Table, Offices::Id)
                        .on_delete(ForeignKeyAction::SetNull),
                )
                .to_owned(),
            )
            .await?;

        // Helpful indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_shipments_client_id")
                    .table(Shipments::Table)
                    .col(Shipments::ClientId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_shipments_current_status")
                    .table(Shipments::Table)
                    .col(Shipments::CurrentStatus)
                    .to_owned(),
            )
            .await?;

        // Shipment Status History
        manager
            .create_table(
                Table::create()
                    .table(ShipmentStatusHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::ShipmentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::FromStatus)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::ToStatus)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::ChangedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::ActorUserId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::OfficeId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ShipmentStatusHistory::Notes)
                            .string()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_status_history_shipment")
                            .from(ShipmentStatusHistory::Table, ShipmentStatusHistory::ShipmentId)
                            .to(Shipments::Table, Shipments::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_status_history_actor_user")
                            .from(ShipmentStatusHistory::Table, ShipmentStatusHistory::ActorUserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_status_history_office")
                            .from(ShipmentStatusHistory::Table, ShipmentStatusHistory::OfficeId)
                            .to(Offices::Table, Offices::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_status_history_shipment_changed_at")
                    .table(ShipmentStatusHistory::Table)
                    .col(ShipmentStatusHistory::ShipmentId)
                    .col(ShipmentStatusHistory::ChangedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ShipmentStatusHistory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Shipments::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Clients::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EmployeeOffices::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Offices::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Employees::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    CreatedAt,
}

#[derive(Iden)]
enum Roles {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum UserRoles {
    Table,
    UserId,
    RoleId,
}

#[derive(Iden)]
enum Employees {
    Table,
    Id,
    UserId,
    FullName,
}

#[derive(Iden)]
enum Offices {
    Table,
    Id,
    Name,
    City,
    Address,
}

#[derive(Iden)]
enum EmployeeOffices {
    Table,
    EmployeeId,
    OfficeId,
}

#[derive(Iden)]
enum Clients {
    Table,
    Id,
    Name,
    Phone,
    Email,
}

#[derive(Iden)]
enum Shipments {
    Table,
    Id,
    ClientId,
    CurrentStatus,
    CurrentOfficeId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum ShipmentStatusHistory {
    Table,
    Id,
    ShipmentId,
    FromStatus,
    ToStatus,
    ChangedAt,
    ActorUserId,
    OfficeId,
    Notes,
}
