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
mod handler;
mod log;
mod model;
mod sync;
mod util;

// TODO 优化返回
// TODO 写入过程异步化
// TODO 使用 base58 来支持更多的 app 和 api
// TODO 支持 app api 名称 包括 连字符、下划线、数字、字母 句号 波浪线 中文？
// TODO 限制 app 和 api 的长度为 16 个字符

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
