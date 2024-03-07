//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "t_capital_rule")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub rule_id: String,
    pub rule_url: String,
    pub rule_source: String,
    pub rule_type: String,
    pub special: i8,
    pub special_name: String,
    pub reconciliation_method: String,
    pub reconciliation_classification: String,
    pub online: i8,
    pub create_at: DateTimeUtc,
    pub update_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
