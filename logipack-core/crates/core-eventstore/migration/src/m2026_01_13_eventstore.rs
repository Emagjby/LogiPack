use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Streams
        manager
            .create_table(
                Table::create()
                    .table(Streams::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Streams::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Streams::Kind).string().not_null())
                    .col(ColumnDef::new(Streams::HeadHash).binary().null())
                    .col(
                        ColumnDef::new(Streams::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Packages
        manager
            .create_table(
                Table::create()
                    .table(Packages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Packages::Hash)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Packages::StreamId).uuid().not_null())
                    .col(ColumnDef::new(Packages::PrevHash).binary().null())
                    .col(ColumnDef::new(Packages::Seq).big_integer().not_null())
                    .col(ColumnDef::new(Packages::EventType).string().not_null())
                    .col(ColumnDef::new(Packages::Scb).binary().not_null())
                    .col(
                        ColumnDef::new(Packages::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_packages_stream")
                            .from(Packages::Table, Packages::StreamId)
                            .to(Streams::Table, Streams::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Constrains
        manager
            .drop_index(Index::drop().name("ux_packages_stream_seq").to_owned())
            .await
            .ok();
        manager
            .create_index(
                Index::create()
                    .name("ux_packages_stream_seq")
                    .table(Packages::Table)
                    .col(Packages::StreamId)
                    .col(Packages::Seq)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("ix_packages_stream_seq").to_owned())
            .await
            .ok();
        manager
            .create_index(
                Index::create()
                    .name("ix_packages_stream_seq")
                    .table(Packages::Table)
                    .col(Packages::StreamId)
                    .col(Packages::Seq)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("ix_packages_prev_hash").to_owned())
            .await
            .ok();
        manager
            .create_index(
                Index::create()
                    .name("ix_packages_prev_hash")
                    .table(Packages::Table)
                    .col(Packages::StreamId)
                    .col(Packages::PrevHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Packages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Streams::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Streams {
    Table,
    Id,
    Kind,
    HeadHash,
    CreatedAt,
}

#[derive(Iden)]
enum Packages {
    Table,
    Hash,
    StreamId,
    PrevHash,
    Seq,
    EventType,
    Scb,
    CreatedAt,
}
