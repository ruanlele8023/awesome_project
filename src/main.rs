use config::{cfg, db};

mod config;
mod router;
mod controller;
mod data;
mod service;
mod entity;
mod util;

#[tokio::main]
async fn main() {
    let config_str = String::from("/Users/bytedance/RustroverProjects/awesome_project/src/config.toml");
    cfg::init(&config_str);
    db::init(cfg::config()).await;
    config::serve::serve().await;
}
