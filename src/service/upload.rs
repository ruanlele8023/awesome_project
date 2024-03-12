use axum_extra::extract::Multipart;
use crate::data::result::response::{ApiErr, ApiOK};
use crate::data::result::response::Result;
use std::str;

pub async fn add(mut multipart: Multipart) -> Result<ApiOK<String>> {
    let file_option = multipart.next_field().await.unwrap();
    if file_option.is_none() {
        return  Ok(ApiOK(Some(String::from("abcds"))));
    }

    let data = file_option.unwrap().bytes().await.unwrap();

    Ok(ApiOK(Some(String::from(str::from_utf8(&data).unwrap()))))
}