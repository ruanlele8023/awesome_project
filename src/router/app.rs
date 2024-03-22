use axum::Router;
use axum::routing::{get, post};
use crate::controller;
use askama::Template;
use axum::response::Html;

pub fn init() -> Router{

    let table_router = Router::new()
        .route("/get", get(controller::table_info_controller::detail))
        .route("/insert", post(controller::table_info_controller::insert));

    let upload_router = Router::new()
        .route("/add", post(controller::upload_controller::add));

    let plant_uml_router = Router::new()
        .route("/generate", post(controller::plant_uml_controller::generate));

    Router::new()
        .route("/hello", get(hello))
        .route("/test", get(test))
        .nest("/table_info", table_router)
        .nest("/upload", upload_router)
        .nest("/plant_uml", plant_uml_router)
}

async fn hello() -> String {
    String::from("hello axum")
}

async fn test() -> Result<Html<String>, String> {
    let name = String::from("axum中文网");
    let tpl = IndexTemplate { name };
    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "example.html")]
pub struct IndexTemplate {
    pub name: String,
}

async fn table_info_get() -> String {
    String::from("hello table info")
}