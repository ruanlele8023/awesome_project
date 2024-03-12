use std::collections::HashMap;
use sea_orm::{ColumnTrait, Condition, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set};
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::config::db;
use crate::data::result::response::{ApiErr, ApiOK};
use crate::data::result::response::Result;
use crate::entity::{prelude::*, t_table_info};
use crate::entity::t_table_info::Model;
use crate::util;

#[derive(Debug, Serialize)]
pub struct TableInfoListResp {
    pub total: u64,
    pub list: Vec<Model>,
}

#[derive(Debug, Serialize)]
pub struct CreateResp {
    pub id: u64,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateReq {
    #[validate(length(min = 1, message = "domain 不能为空"))]
    pub domain: String,
    pub sub_domain: String,
    #[validate(length(min = 1, message = "table_info_name 不能为空"))]
    pub table_info_name: String,
    pub table_fields: Option<String>,
}

pub async fn insert(req: CreateReq) -> Result<ApiOK<CreateResp>> {
    match TTableInfo::find()
        .filter(
            Condition::all()
                .add(t_table_info::Column::TableInfoName.eq(req.table_info_name.clone()))
                .add(t_table_info::Column::Domain.eq(req.domain.clone()))
        ).count(db::conn())
        .await
    {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => {
            if v > 0 {
                return Err(ApiErr::ErrPerm(Some("该编号已被使用".to_string())));
            }
        }
    }

    let now = chrono::Local::now();

    let am = t_table_info::ActiveModel {
        table_info_name: Set(req.table_info_name),
        sub_domain: Set(req.sub_domain),
        domain: Set(req.domain),
        table_fields: Set(req.table_fields.unwrap_or(String::from(""))),
        create_at: Set(DateTimeUtc::from(now)),
        ..Default::default()
    };

    let last_insert_id = match  TTableInfo::insert(am).exec(db::conn()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err count project");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => v.last_insert_id,
    };

    let resp = CreateResp {
        id : last_insert_id,
    };

    Ok(ApiOK(Some(resp)))
}

pub async fn get(query: HashMap<String, String>) -> Result<ApiOK<TableInfoListResp>> {
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

    let resp = TableInfoListResp {
        total: 0,
        list: table_info_list,
    };
    Ok(ApiOK(Some(resp)))
    // Ok(ApiOK(Some(resp)))
}