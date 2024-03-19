use std::collections::HashMap;
use axum::extract::Query;
use crate::data::result::response::ApiOK;
use crate::service;
use crate::service::table_info::TableInfoListResp;

pub async fn generate(Query(query): Query<HashMap<String, String>>,
) -> crate::data::result::response::Result<ApiOK<TableInfoListResp>> {
    service::table_info::get(query).await
}