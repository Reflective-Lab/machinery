use axum::{Json, Router, routing::get};
use std::sync::Arc;
use {{project}}_app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Arc::new(App::new());

    let router = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route(
            "/truths",
            get({
                let app = app.clone();
                move || async move { Json(app.list_truths().len()) }
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}
