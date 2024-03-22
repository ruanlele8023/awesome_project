use std::collections::{HashMap, HashSet};
use core::default::Default;
use axum::response::Html;
use sea_orm::sea_query::IndexType::Hash;
use validator::HasLen;
use crate::data::result::response::ApiOK;
use crate::data::result::response::Result;
use crate::entity::t_table_info::Model;
use crate::service::dao;
use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct PlantUmlTemplate {
    pub chains: Vec<Chain>,
    pub package_infos: Vec<PackageInfo>
}

pub struct PackageInfo {
    pub domain: String,
    pub table_infos: Vec<TableInfo>
}

#[derive(Clone)]
pub struct TableInfo {
    pub table_name: String,
}

#[derive(Default)]
pub struct Chain {
    pub start: String,
    pub end: String,
    pub arrow: String,
    pub existed: bool,
    pub tag1: String,
    pub tag2: String,
    pub tag3: String,
    pub tag4: String,
    pub rule_id: String,
    pub fields_flag: bool,
    pub fields: Vec<Field>
}

// impl Default for Chain {
//     fn default() -> Self {
//         Self {
//             start: String::default(),
//             end: String::default(),
//             pub arrow: String::default(),
//             pub existed: bool::default(),
//             pub tag1: String::default(),
//             pub tag2: String,
//             pub tag3: String,
//             pub tag4: String,
//             pub rule_id: String,
//             pub fields_flag: bool,
//             pub fields: Vec< crate::service::plant_uml::Field >
//         }
//     }
// }

pub struct Field {
    pub table_name: String,
    pub fields: Vec<String>
}

pub async fn generate_plant_uml(query: HashMap<String, String>)  -> Result<ApiOK<String>> {

    let capital_rules = dao::atomic_capital_rule::query(query).await;

    let capital_rules_ids  = capital_rules.clone().into_iter().map(|x1| {
        x1.rule_id
    }).collect::<Vec<_>>();

    let rule_id_to_info = capital_rules.clone().into_iter().map(|x1| {
        (x1.rule_id.clone(), x1.clone())
    }).collect::<HashMap<String, crate::entity::t_capital_rule::Model>>();

    let capital_rules_chains = dao::atomic_capital_rule_chain::query_by_rule_ids(capital_rules_ids).await;

    let mut rule_id_to_rule_chains : HashMap<String, Vec<crate::entity::t_capital_rule_chain::Model>> = HashMap::new();

    for item in &capital_rules_chains {
        rule_id_to_rule_chains.entry(item.rule_id.clone()).or_default().push(item.clone());
    }

    let capital_table_ids = capital_rules_chains.clone().into_iter().map(|x1| {
        x1.table_info_id
    }).collect::<Vec<_>>();

    let capital_table_ids_set: HashSet<i64> = capital_table_ids.into_iter().collect();

    let table_infos = dao::atomic_table_info::query_by_ids(capital_table_ids_set.into_iter().collect()).await;

    let mut table_id_to_info : HashMap<u64, Model> = HashMap::new();

    for item in &table_infos {
        // table_id_to_info.entry(item.id.clone()).or_default().push(item.clone())
        table_id_to_info.insert(item.id.clone(), item.clone());
    }

    let package_infos = load_package_infos(table_id_to_info.clone());
    let chains = load_chains(rule_id_to_rule_chains, rule_id_to_info, table_id_to_info.clone());

    let plant_uml_template = PlantUmlTemplate{
        package_infos,
        chains,
    };


    match  plant_uml_template.render() {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Ok(ApiOK(None));
        }
        Ok(v) => Ok(ApiOK(Some(v))),
    }
}

pub fn load_chains(rule_id_to_chains: HashMap<String, Vec<crate::entity::t_capital_rule_chain::Model>>,
                   mut rule_id_to_info: HashMap<String, crate::entity::t_capital_rule::Model>,
                   mut table_id_to_info: HashMap<u64, Model>) -> Vec<Chain> {
    let mut chains_lack_collection: HashMap<String, HashMap<String, HashMap<String, bool>>> = HashMap::new();
    let mut res: Vec<Chain> = Vec::new();

    for (key, chains) in &rule_id_to_chains {
        let rule_info = rule_id_to_info.get_mut(key).unwrap();
        for  (idx1, value) in  chains.iter().enumerate() {
            if chains.length() != 1 && idx1 == chains.length() as usize - 1 {
                continue;
            }

            let mut chain: Chain = Default::default();
            chain.rule_id = idx1.to_string();
            // chain.start = table_id_to_info.get_mut((*value.table_info_id) as u64);
            if chains.length() == 1 {
                chain.end = chain.start.clone()
            } else {
                // chain.end = table_id_to_info.get(*chains.get(idx1  + 1).unwrap().table_info_id.clone()).unwrap().table_info_name.clone()
            }
            res.push(chain)
        }
    }
    res
}

pub fn load_package_infos(table_id_to_info: HashMap<u64, Model>) -> Vec<PackageInfo> {
    let mut domain_to_package_infos: HashMap<String, Vec<TableInfo>> = HashMap::new();

    for (key, value) in &table_id_to_info {
       domain_to_package_infos.entry(value.domain.clone()).or_default().push(TableInfo{
           table_name: value.table_info_name.clone()
       })
    }

    let mut res: Vec<PackageInfo> = Vec::new();

    for (key, value) in domain_to_package_infos {
        res.push(PackageInfo{
            domain: key.clone(),
            table_infos: value.clone()
        })
    }
    res
}