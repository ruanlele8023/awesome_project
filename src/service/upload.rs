use crate::data::result::response::ApiOK;
use crate::data::result::response::Result;

pub async fn add() -> Result<ApiOK<()>> {
    Ok(ApiOK(None))
}