use anyhow::{Ok, Result};
use axum::{
    routing::{get, post},
    Router,
};
use common::{init, CONTEXT};
use config::CONFIG;
use tracing::info;

use crate::{
    controller::{api as ApiController, app as AppController},
    sync::db_sync,
};

mod common;
mod config;
mod controller;
mod db;
mod log;
mod model;
mod service;
mod sync;
mod util;
mod handler;

// TODO 获取 app 下所有 api 的总访问次数
// TODO 更换更小的粒度, 尝试是否能新增一个使用 HashMap 一层结构 用 RwLock控制所有 app_api 的访问次数

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    init_log!();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api", post(AppController::add))
        .route(
            "/api/:app",
            get(AppController::get).post(ApiController::add),
        )
        .route(
            "/api/:app/:api",
            get(ApiController::get).post(ApiController::post),
        );

    // 数据库同步任务
    tokio::spawn(async {
        db_sync().await;
    });

    info!("Server started at {}", CONFIG.server_url);
    axum::Server::bind(&CONFIG.server_url.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
