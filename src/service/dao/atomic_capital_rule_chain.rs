use sea_orm::{ColumnTrait, EntityTrait, Order};
use sea_orm::sea_query::Mode;
use crate::config::db;
use crate::entity::prelude::{TCapitalRuleChain, TTableInfo};
use crate::entity::{t_capital_rule_chain, t_table_info};
use crate::entity::t_capital_rule_chain::Model;
use sea_orm::QueryFilter;

pub async fn insert(am: t_capital_rule_chain::ActiveModel) -> Option<u64> {
    match TCapitalRuleChain::insert(am).exec(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err count project");
            return None;
        }
        Ok(v) => Some(v.last_insert_id),
    }
}

pub async fn query_by_rule_ids(rule_ids: Vec<String>) -> Vec<Model> {
    let mut builder = TCapitalRuleChain::find();

    match builder.filter(t_capital_rule_chain::Column::RuleId.is_in(rule_ids.clone()))
        .all(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Vec::new();
        }
        Ok(v) => v,
    }
}