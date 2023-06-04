use anyhow::{Ok, Result};
use axum::{
    routing::{get, post},
    Router,
};
use common::{init, CONTEXT};
use config::CONFIG;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::{
    controller::{api as Api, app as App},
    sync::db_sync,
};

mod common;
mod config;
mod controller;
mod db;
mod handler;
mod log;
mod model;
mod resp;
mod sync;
mod util;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    init_log!();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api", post(App::add))
        .route("/api/:app", get(App::get).post(Api::add))
        .route("/api/:app/:api", get(Api::get).post(Api::post))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // 数据库同步任务
    // Database synchronization task
    tokio::spawn(async {
        db_sync().await;
    });

    info!("Server started at {}", CONFIG.server_url);
    axum::Server::bind(&CONFIG.server_url.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
