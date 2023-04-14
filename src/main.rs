use std::{
    collections::{hash_map, HashMap, HashSet},
    time::Duration,
};

use anyhow::{Ok, Result};
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use common::{init, CONTEXT};
use config::CONFIG;
use dto::{AddApiDTO, AddAppDTO, GetApiDTO, PostApiDTO};
use tracing::info;

use crate::db::make_table;

mod common;
mod config;
mod db;
mod dto;
mod log;
mod model;
mod util;
mod vo;

// TODO 统一返回体
// TODO 获取 app 下所有 api 的总访问次数
// TODO 使用 HashMap::with_capacity() 以避免提前分配内存

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    init_log!();

    let app = Router::new()
        .route("/", get(index))
        .route("/api", post(add_app))
        .route("/api/:app", get(get_api).post(post_api).put(add_api));

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

async fn add_app(Json(AddAppDTO { name }): Json<AddAppDTO>) -> String {
    // 检测 app name 是否合法
    // Check if app name is valid
    if !util::is_valid(&name) {
        return "Invalid app name".to_owned();
    };

    match context!().apps.read().get(&name) {
        None => {
            {
                context!().apps.write().insert(name.clone());
            }
            {
                context!().wait_app.write().push(name);
            }
            "Success".to_owned()
        }
        _ => "App already exists".to_owned(),
    }
}

async fn add_api(Path(app): Path<String>, Json(AddApiDTO { api }): Json<AddApiDTO>) -> String {
    // 检测 api name 是否合法
    // Check if api name is valid
    if !util::is_valid(&api) {
        return "Invalid api name".to_owned();
    };

    // TODO 校验锁的粒度
    match context!().apps.read().get(&app) {
        Some(_) => {
            {
                context!()
                    .apis
                    .write()
                    .get_mut(&app)
                    .unwrap()
                    .insert(api.clone(), 0);
            }
            {
                context!()
                    .wait_api
                    .write()
                    .get_mut(&app)
                    .unwrap()
                    .insert(api);
            }
            "Success".to_owned()
        }
        None => "App not found".to_owned(),
    }
}

// TODO 待优化 统一返回体 流处理
// 获取 api 访问数量
// Get api access count
async fn get_api(Path(app): Path<String>, Json(GetApiDTO { apis }): Json<GetApiDTO>) -> String {
    match context!().apis.read().get(&app) {
        Some(count) => {
            let mut res = HashMap::new();
            for api in apis.iter() {
                res.insert(api.clone(), count.get(api));
            }
            serde_json::to_string(&res).unwrap()
        }
        None => "App not found".to_owned(),
    }
}

// 新增记录
// Add record
async fn post_api(Path(app): Path<String>, Json(PostApiDTO { apis }): Json<PostApiDTO>) -> String {
    match context!().apps.read().get(&app) {
        Some(_) => {
            {
                // 更新内存中的 api 访问次数
                // Update api access count in memory
                {
                    let mut lock = context!().apis.write();
                    let lock = lock.get_mut(&app).unwrap();
                    for api in apis.iter() {
                        lock.entry(api.clone()).and_modify(|e| *e += 1);
                    }
                }

                // 更新待添加到数据库中的 api 访问记录
                // Update api access count to be added to database
                {
                    let mut lock = context!().wait_record.write();
                    let lock = lock.get_mut(&app).unwrap();
                    for api in apis.iter() {
                        lock.get_mut(api).unwrap().push(
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_secs() as i64,
                        );
                    }
                }
            }
            // TODO 返回更新后的数据
            "Success".to_owned()
        }
        None => "App not found".to_owned(),
    }
}

// TODO 支持设置同步间隔
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

        // TODO 原子化 新建app表
        for app in wait_app.iter() {
            make_table(app).await;
        }

        // 获取需要新增的api
        // Get new api
        let wait_api = {
            let mut lock = context!().wait_api.write();
            let wait_api = lock.clone();
            lock.clear();
            wait_api
        };

        // TODO 在 app 表中新增 api 记录

        // 获取需要新增的记录
        // Get new record
        let wait_record = {
            let mut lock = context!().wait_record.write();
            let wait_record = lock.clone();
            lock.clear();
            wait_record
        };

        // 合并相同记录
        // Merge the same record
        let mut records: HashMap<&String, HashMap<&String, HashMap<&i64, i32>>> = HashMap::new();
        for (app, apis) in wait_record.iter() {
            let mut apis_merged: HashMap<&String, HashMap<&i64, i32>> = HashMap::new();
            for (api, times) in apis.iter() {
                apis_merged.insert(
                    api,
                    times.iter().fold(HashMap::new(), |mut acc, time| {
                        acc.entry(time).and_modify(|e| *e += 1).or_insert(1);
                        acc
                    }),
                );
            }
            records.insert(app, apis_merged);
        }

        // 需要更新Api的值
        // Api value to be updated
        let api_update: HashMap<&String, HashMap<&String, i32>> = records
            .iter()
            .map(|(app, records)| {
                let apis: HashMap<&String, i32> = records
                    .iter()
                    .map(|(api, times)| {
                        let count: i32 = times.iter().map(|(_, count)| count).sum();
                        (*api, count)
                    })
                    .collect();
                (*app, apis)
            })
            .collect();

        // TODO 更新记录表

        // TODO 更新api表
    }
}
