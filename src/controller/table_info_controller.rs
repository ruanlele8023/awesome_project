use std::collections::HashMap;

use axum::extract::Query;
use axum::Json;
use axum_extra::extract::WithRejection;

use crate::data::result::rejection::IRejection;
use crate::data::result::response::ApiOK;
use crate::data::result::response::Result;
use crate::service;
use crate::service::table_info::{CreateReq, CreateResp, TableInfoListResp};

pub async fn detail(Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<TableInfoListResp>> {
    service::table_info::get(query).await
}

pub async fn insert(WithRejection(Json(req), _): IRejection<Json<CreateReq>>,
) -> Result<ApiOK<CreateResp>> {
    service::table_info::insert(req).await
}