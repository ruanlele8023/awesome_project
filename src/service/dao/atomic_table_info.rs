use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect};
use crate::config::db;
use crate::data::result::response::{ApiErr, ApiOK};
use crate::entity::prelude::{TCapitalRuleChain, TTableInfo};
use crate::entity::{t_capital_rule_chain, t_table_info};
use crate::entity::t_table_info::Model;
use crate::service::table_info;
use crate::service::table_info::TableInfoListResp;

pub async fn get_by_name_domain(table_name: Option<String>, domain: Option<String>) -> Vec<Model> {
    let mut builder = TTableInfo::find();

    if table_name.is_some() {
        builder = builder.filter(t_table_info::Column::TableInfoName.eq(table_name.clone()))
    }

    if domain.is_some() {
        builder = builder.filter(t_table_info::Column::Domain.eq(domain.clone()))
    }

    match builder
        .order_by(t_table_info::Column::Id, Order::Desc)
        .offset(0)
        .limit(1)
        .all(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Vec::new();
        }
        Ok(v) => v,
    }
}

pub async fn insert(am: t_table_info::ActiveModel) -> Option<u64> {
     match  TTableInfo::insert(am).exec(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err count project");
            return None;
        }
        Ok(v) => Some(v.last_insert_id),
    }
}

pub async fn query_by_ids(ids: Vec<i64>) -> Vec<Model> {
    let mut builder = TTableInfo::find();

    match builder.filter(t_table_info::Column::Id.is_in(ids.clone()))
        .all(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Vec::new();
        }
        Ok(v) => v,
    }
}