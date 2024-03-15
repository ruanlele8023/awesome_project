use sea_orm::EntityTrait;
use crate::config::db;
use crate::entity::prelude::TCapitalRuleChain;
use crate::entity::t_capital_rule_chain;

pub async fn insert(am: t_capital_rule_chain::ActiveModel) -> Option<u64> {
    match  TCapitalRuleChain::insert(am).exec(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err count project");
            return None;
        }
        Ok(v) =>  Some(v.last_insert_id),
    }
}