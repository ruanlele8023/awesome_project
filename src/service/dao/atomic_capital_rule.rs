use sea_orm::EntityTrait;
use crate::config::db;
use crate::entity::prelude::TCapitalRule;
use crate::entity::t_capital_rule;

pub async fn insert(am: t_capital_rule::ActiveModel) -> Option<u64> {
    match  TCapitalRule::insert(am).exec(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err count project");
            return None;
        }
        Ok(v) =>  Some(v.last_insert_id),
    }
}