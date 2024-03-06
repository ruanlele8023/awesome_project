use std::collections::HashMap;
use axum::extract::{Query};
use crate::data::result::response::ApiOK;
use crate::data::result::response::Result;
use crate::service;
use crate::service::table_info::TableInfoResp;

pub async fn detail(Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<TableInfoResp>> {
    service::table_info::get(query).await
}