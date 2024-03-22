use std::collections::HashMap;
use axum::extract::Query;
use crate::data::result::response::ApiOK;
use crate::service;
use crate::data::result::response::Result;

pub async fn generate(Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<String>> {
    service::plant_uml::generate_plant_uml(query).await
}