use std::collections::HashMap;
use sea_orm::{EntityTrait, Order, QueryOrder, QuerySelect};
use crate::config::db;
use crate::entity::prelude::{TCapitalRule, TTableInfo};
use crate::entity::{t_capital_rule, t_table_info};
use crate::entity::t_capital_rule::Model;

pub async fn insert(am: t_capital_rule::ActiveModel) -> Option<u64> {
    match  TCapitalRule::insert(am).exec(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err count project");
            return None;
        }
        Ok(v) =>  Some(v.last_insert_id),
    }
}

pub async fn query(query: &HashMap<String, String>) -> Vec<Model> {
    let mut builder = TCapitalRule::find();
    match builder
        .order_by(t_capital_rule::Column::Id, Order::Desc)
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