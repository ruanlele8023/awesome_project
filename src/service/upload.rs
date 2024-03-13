use std::collections::HashMap;
use axum_extra::extract::Multipart;
use crate::data::result::response::{ApiErr, ApiOK};
use crate::data::result::response::Result;
use std::str;

pub async fn add(mut multipart: Multipart) -> Result<ApiOK<Vec<HashMap<String, String>>>> {
    let file_option = multipart.next_field().await.unwrap();
    if file_option.is_none() {
        return  Ok(ApiOK(None));
    }

    let data = file_option.unwrap().bytes().await.unwrap();

    let temp_data = String::from(str::from_utf8(&data).unwrap());

    let mut reader = csv::Reader::from_reader(temp_data.as_bytes());

    let mut vec: Vec<HashMap<String, String>> = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("rule_id"),  String::from(&record[0]));
        map.insert(String::from("rule_url"), String::from(&record[1]));
        vec.push(map);
    }

    Ok(ApiOK(Some(vec)))
}