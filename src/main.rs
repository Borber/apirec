use std::{collections::HashMap, time::Duration};

use anyhow::{Ok, Result};
use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};
use common::{init, CONTEXT};
use config::CONFIG;
use tracing::info;

use crate::db::make_table;

mod common;
mod config;
mod db;
mod log;
mod model;

// TODO 统一返回体

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    init_log!();

    let app = Router::new()
        .route("/", get(index))
        .route("/api/add/:name", post(add_app))
        .route("/api/data/:name", get(app_get).post(app_post));

    // 数据库同步任务
    tokio::spawn(async {
        dbsync().await;
    });

    info!("Server started at {}", CONFIG.server_url);
    axum::Server::bind(&CONFIG.server_url.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn index() -> &'static str {
    "Hello, World!"
}

// TODO 检测是否应该使用 {} 来提前释放锁
async fn add_app(Path(name): Path<String>) -> String {
    // 检测 app name 是否合法
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c.eq(&'_'))
    {
        return "Invalid app name".to_owned();
    }

    match context!().apps.read().get(&name) {
        None => {
            {
                context!().counts.write().insert(name.clone(), 0);
            }
            {
                context!().wait_app.write().push(name);
            }
            "Success".to_owned()
        }
        _ => "App already exists".to_owned(),
    }
}

// 获取 app 访问记录
async fn app_get(Path(name): Path<String>) -> String {
    match context!().counts.read().get(&name) {
        Some(count) => count.to_string(),
        None => "App not found".to_owned(),
    }
}

// 新增记录
// Add record
async fn app_post(Path(name): Path<String>) -> String {
    match context!().apps.read().get(&name) {
        Some(_) => {
            {
                context!().wait_record.write().push((
                    name.clone(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs() as i64,
                ));
            }
            {
                context!()
                    .counts
                    .write()
                    .entry(name)
                    .and_modify(|e| *e += 1)
                    .or_insert(1)
                    .to_string()
            }
        }
        None => "App not found".to_owned(),
    }
}

// 数据库同步
async fn dbsync() {
    info!("Database sync started");
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;

        // 获取需要新增的 app
        // Get new app
        let wait_app = {
            let mut lock = context!().wait_app.write();
            let wait_app = lock.clone();
            lock.clear();
            wait_app
        };

        // TODO 原子化
        for app in wait_app.iter() {
            make_table(app).await;
        }

        // 将新增的 app 添加到 apps
        // Add new app to apps
        {
            context!().apps.write().extend(wait_app)
        }

        // 获取需要新增的记录
        // Get new record
        let wait_record = {
            let mut lock = context!().wait_record.write();
            let wait_record = lock.clone();
            lock.clear();
            wait_record
        };

        // 合并同一App的记录
        // Merge the same app record
        let wait_record: Vec<(String, i64, i32)> = wait_record
            .into_iter()
            .fold(HashMap::new(), |mut acc, (app, time)| {
                *acc.entry((app, time)).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .map(|((string, time), count)| (string, time, count))
            .collect();

        // 所有需要更新的 Apps
        // All apps need to update
        let app_set = wait_record
            .iter()
            .map(|(table, _, _)| table.clone())
            .collect::<std::collections::HashSet<String>>();

        // TODO 原子化
        for (table, time, count) in wait_record.into_iter() {
            db::add_rec(table, time, count).await;
        }

        // 获取需要更新的记录
        let counts = { context!().counts.read().clone() };
        let counts = app_set
            .into_iter()
            .map(|app| (*counts.get(&app).unwrap(), app))
            .collect::<Vec<_>>();

        // TODO 原子化
        for (count, table) in counts.iter() {
            db::update_count(table, count).await;
        }
    }
}
