use config::{cfg, db};

mod config;
mod router;
mod controller;
mod data;
mod service;


#[tokio::main]
async fn main() {
    // let app = Router::new().route("/", get(handler));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // // run it with hyper on localhost:3000
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    let config_str = String::from("/Users/bytedance/RustroverProjects/awesome_project/src/config.toml");
    cfg::init(&config_str);
    db::init(cfg::config()).await;
    config::serve::serve().await;
}