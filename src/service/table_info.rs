use std::collections::HashMap;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, QueryTrait};
use serde::Serialize;
use crate::config::db;
use crate::data::result::response::{ApiErr, ApiOK};
use crate::data::result::response::Result;
use crate::entity::{prelude::*,t_table_info};
use crate::entity::t_table_info::Model;
use crate::util;

#[derive(Debug, Serialize)]
pub struct TableInfoResp {
    pub list: Vec<Model>,
}

pub async fn get(query: HashMap<String, String>) -> Result<ApiOK<TableInfoResp>> {
    let mut builder = TTableInfo::find();

    // let resp = TableInfoResp {
    //     id: 0,
    // };
    if let Some(id) = query.get("id") {
        if let Ok(v) = id.parse::<u64>() {
            builder = builder.filter(t_table_info::Column::Id.eq(v))
        }
    }

    let (offset, limit) = util::page::query_page(&query);

    // let mut total: i64 = 0;

    let table_info_list = match builder
        .order_by(t_table_info::Column::Id, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => v,
    };

    let resp = TableInfoResp {
        list : table_info_list,
    };
    Ok(ApiOK(Some(resp)))
    // Ok(ApiOK(Some(resp)))
}