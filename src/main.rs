use anyhow::{Ok, Result};
use common::{init, CONTEXT};
use config::CONFIG;
use ntex::{http::Method, web};
use ntex_cors::Cors;
use tracing::info;

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

#[ntex::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    init_log!();

    // 数据库同步任务
    // Database synchronization task
    tokio::spawn(async {
        db_sync().await;
    });

    info!("Server started at {}", CONFIG.server_url);
    web::HttpServer::new(|| {
        web::App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec![Method::GET, Method::POST])
                    .max_age(3600)
                    .finish(),
            )
            .route("/", web::get().to(|| async { "Hello, world!" }))
            .service(App::add)
            .service(App::get)
            .service(Api::add)
            .service(Api::get)
            .service(Api::post)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;
    Ok(())
}
