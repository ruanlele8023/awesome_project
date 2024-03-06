use std::collections::HashMap;
use serde::Serialize;
use crate::data::result::response::ApiOK;
use crate::data::result::response::Result;

#[derive(Debug, Serialize)]
pub struct TableInfoResp {
    pub id: u64,
}

pub async fn get(query: HashMap<String, String>) -> Result<ApiOK<TableInfoResp>> {

    let resp = TableInfoResp {
        id: 0,
    };

    Ok(ApiOK(Some(resp)))
}