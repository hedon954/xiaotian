use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Feed::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Feed::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Feed::Name).string().not_null())
                    .col(ColumnDef::new(Feed::FeedType).string().not_null())
                    .col(ColumnDef::new(Feed::FeedUrl).string().not_null())
                    .col(ColumnDef::new(Feed::Description).string().null())
                    .col(ColumnDef::new(Feed::Category).string().null())
                    .col(ColumnDef::new(Feed::Status).string().not_null().default("loading"))
                    .col(ColumnDef::new(Feed::Icon).string().null())
                    .col(ColumnDef::new(Feed::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Feed::LastUpdated).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Feed::SummaryCount).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Feed::FeedUrl)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Feed::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Feed {
    Table,
    Id,
    Name,
    FeedType,
    FeedUrl,
    Description,
    Category,
    Status,
    Icon,
    CreatedAt,
    LastUpdated,
    SummaryCount,
}