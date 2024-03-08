use axum::Router;
use axum::routing::{get, post};
use crate::controller;

pub fn init() -> Router{

    let table_router = Router::new()
        .route("/get", get(controller::table_info_controller::detail))
        .route("/insert", post(controller::table_info_controller::insert));

    Router::new()
        .route("/hello", get(hello))
        .nest("/table_info", table_router)
}

async fn hello() -> String {
    String::from("hello axum")
}

async fn table_info_get() -> String {
    String::from("hello table info")
}