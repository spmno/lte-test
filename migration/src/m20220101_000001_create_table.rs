use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "create_locations_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(entities::locations::Entity)
                    .col(ColumnDef::new(entities::locations::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(entities::locations::Longitude).double().not_null())
                    .col(ColumnDef::new(entities::locations::Latitude).double().not_null())
                    .col(ColumnDef::new(entities::locations::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entities::locations::Entity).to_owned())
            .await
    }
}