use std::collections::HashMap;
use axum_extra::extract::Multipart;
use crate::data::result::response::{ApiErr, ApiOK};
use crate::data::result::response::Result;
use std::str;
use axum::response::IntoResponse;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::entity::{t_capital_rule, t_capital_rule_chain, t_table_info};
use crate::service::{dao, table_info};
use crate::service::table_info::CreateReq;

pub async fn add(mut multipart: Multipart) -> Result<ApiOK<()>> {
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
        map.insert(String::from("rule_type"), String::from(&record[2]));
        map.insert(String::from("rule_source"), String::from(&record[3]));
        map.insert(String::from("domain1"), String::from(&record[4]));
        map.insert(String::from("table1"), String::from(&record[5]));
        vec.push(map);
    }

    deal_with_records(vec).await;

    Ok(ApiOK(None))
}

#[derive(Debug, Clone)]
struct StandardReq {
    pub rule_id: String,
    pub rule_url: String,
    pub rule_source: String,
    pub rule_type: String,
    pub special: bool,
    pub special_name: String,
    pub reconciliation_method: String,
    pub reconciliation_classification: String,
    pub online: bool,
    pub rule_chains: Vec<RuleChain>
}

#[derive(Debug, Clone)]
struct RuleChain {
    pub domain: String,
    pub sub_domain: Option<String>,
    pub table_fields: Option<String>,
    pub table_info_name: String,
}

pub async fn deal_with_records(records: Vec<HashMap<String, String>>) {
    for record in records {
        let standard_req = standard_request(record);
        deal_with_standard_req(standard_req).await;
    }
}

fn standard_request(map: HashMap<String, String>) -> StandardReq {
    let mut rule_chains :Vec<RuleChain> = Vec::new();
    rule_chains.push(RuleChain {
        domain: map.get("domain1").unwrap().clone(),
        table_info_name: map.get("table1").unwrap().clone(),
        sub_domain: None,
        table_fields: None,
    });

    if map.get("domain2").unwrap_or(&String::from("")) != "" {
        rule_chains.push(RuleChain {
            domain: map.get("domain2").unwrap().clone(),
            table_info_name: map.get("table2").unwrap().clone(),
            sub_domain: None,
            table_fields: None,
        });
    };

    if map.get("domain3").unwrap_or(&String::from("")) != "" {
        rule_chains.push(RuleChain {
            domain: map.get("domain3").unwrap().clone(),
            table_info_name: map.get("table3").unwrap().clone(),
            sub_domain: None,
            table_fields: None
        })
    };

    StandardReq {
        rule_id: map.get("rule_id").unwrap().clone(),
        rule_url: map.get("rule_url").unwrap().clone(),
        rule_source: map.get("rule_source").unwrap().clone(),
        rule_type: map.get("rule_type").unwrap().clone(),
        special: map.get("special").unwrap().is_empty(),
        special_name: map.get("special_name").unwrap().clone(),
        reconciliation_method: map.get("reconciliation_method").unwrap().clone(),
        reconciliation_classification: map.get("reconciliation_classification").unwrap().clone(),
        online: map.get("online").unwrap().is_empty(),
        rule_chains,
    }
}

pub async fn deal_with_standard_req(standard_req: StandardReq) {

    let now = chrono::Local::now();

    let am2 = t_capital_rule::ActiveModel {
        rule_url: Set(standard_req.rule_id.clone()),
        create_at: Set(DateTimeUtc::from(now)),
        ..Default::default()
    };

    _ = dao::atomic_capital_rule::insert(am2);

    for mut chain in standard_req.rule_chains {
        let table_info = dao::atomic_table_info::get_by_name_domain(Some(chain.table_info_name.clone()), Some(chain.domain.clone())).await;

        let mut table_info_id : u64 = 0;

        if !table_info.is_empty() {
            table_info_id = table_info.get(0).unwrap().id
        }



        let am = t_table_info::ActiveModel {
            table_info_name: Set(chain.table_info_name.clone()),
            sub_domain: Set(String::from("")),
            domain: Set(chain.domain.clone()),
            table_fields: Set(String::from("")),
            create_at: Set(DateTimeUtc::from(now)),
            ..Default::default()
        };

        table_info_id = dao::atomic_table_info::insert(am).await.unwrap();

        let am1 = t_capital_rule_chain::ActiveModel {
            rule_id: Set(standard_req.rule_id.clone()),
                start: Set(1),
                end: Set(1),
                ..Default::default()
        };

        _ = dao::atomic_capital_rule_chain::insert(am1).await
    }
}

pub async fn deal_with_record(map: HashMap<String, String>) {
    let mut table_info_req_list : Vec<CreateReq> = Vec::new();
    table_info_req_list.push(CreateReq {
        domain: map.get("domain1").unwrap().clone(),
        table_info_name: map.get("table1").unwrap().clone(),
        sub_domain: String::from(""),
        table_fields: None
    });

    if map.get("domain2").unwrap_or(&String::from("")) != "" {
        table_info_req_list.push(CreateReq {
            domain: map.get("domain2").unwrap().clone(),
            table_info_name: map.get("table2").unwrap().clone(),
            sub_domain: String::from(""),
            table_fields: None
        })
    }

    if map.get("domain3").unwrap_or(&String::from("")) != "" {
        table_info_req_list.push(CreateReq {
            domain: map.get("domain3").unwrap().clone(),
            table_info_name: map.get("table3").unwrap().clone(),
            sub_domain: String::from(""),
            table_fields: None
        })
    }

    let mut table_id_to_table_name: HashMap<String, String> = HashMap::new();

    for mut table_req in table_info_req_list {
        let result = table_info::insert(table_req.clone()).await;
        let table_id = match result {
            Ok(res) => res.0.unwrap().id,
            Err(err) => {
                0
            }
        };
        table_id_to_table_name.insert(table_id.to_string(), table_req.table_info_name.clone());
    }


}