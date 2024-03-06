use crate::config::cfg;
use crate::router::app;

pub async fn serve() {
    let addr= cfg::config().get_int("app.port").unwrap_or(8000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", addr))
        .await
        .unwrap();
    tracing::info!("listening on {}", addr);

    axum::serve(listener, app::init()).await.unwrap();
}