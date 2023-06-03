use anyhow::{Ok, Result};

use salvo::{
    cors::{Any, Cors},
    hyper::Method,
    prelude::*,
};
use tracing::info;

use common::{init, CONTEXT};
use config::CONFIG;

use crate::{
    controller::{api as Api, app as App},
    sync::db_sync,
};

mod common;
mod config;
mod controller;
mod db;
mod log;
mod model;
mod sync;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    init_log!();

    let cors = Cors::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .into_handler();

    let router = Router::with_path("api").hoop(cors).post(App::add).push(
        Router::with_path("<app>")
            .get(App::get)
            .post(Api::add)
            .push(Router::with_path("<api>").get(Api::get).post(Api::post)),
    );

    // 数据库同步任务
    // Database synchronization task
    tokio::spawn(async {
        db_sync().await;
    });

    info!("Server started at {}", CONFIG.server_url);

    let acceptor = TcpListener::new(&CONFIG.server_url).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
