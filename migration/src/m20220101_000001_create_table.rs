use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Position::Table)
                    .if_not_exists()
                    .col(pk_auto(Position::Id))
                    .col(string(Position::SerialNumber))
                    .col(float(Position::Latitude))
                    .col(float(Position::Longitude))
                    .col(string(Position::DeviceType))
                    .col(time(Position::UploadTime))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Position::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Position {
    Table,
    Id,
    SerialNumber,
    Latitude,
    Longitude,
    UploadTime,
    DeviceType,
}