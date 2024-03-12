use axum_extra::extract::Multipart;

use crate::data::result::response::ApiOK;
use crate::data::result::response::Result;
use crate::service;

pub async fn add(mut multipart: Multipart
) -> Result<ApiOK<()>> {
    service::upload::add()
}