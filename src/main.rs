use std::{collections::HashMap, time::Duration};

use anyhow::{Ok, Result};
use axum::{extract::Path, routing::get, Router};
use common::{init, CONTEXT};
use config::CONFIG;

use crate::db::make_table;

mod common;
mod config;
mod db;
mod model;

// TODO log模块引入
// TODO 拆分 get 获取 和post 累加

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    let app = Router::new()
        .route("/", get(index))
        .route("/:app", get(app));

    let task = tokio::spawn(async {
        dbsync().await;
    });

    let server =
        axum::Server::bind(&CONFIG.server_url.parse().unwrap()).serve(app.into_make_service());

    server.await?;
    task.await?;

    Ok(())
}

async fn index() -> &'static str {
    "Hello, World!"
}

async fn app(Path(app): Path<String>) -> String {
    // TODO 安全检查
    context!().dbmsg.write().push((
        app.clone(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64,
    ));

    context!()
        .counts
        .write()
        .entry(app)
        .and_modify(|e| *e += 1)
        .or_insert(1)
        .to_string()
}

async fn dbsync() {
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        let dbmsg = context!().dbmsg.write().clone();
        context!().dbmsg.write().clear();
        let counts: Vec<(String, i64, i32)> = dbmsg
            .into_iter()
            .fold(HashMap::new(), |mut acc, (app, time)| {
                *acc.entry((app, time)).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .map(|((string, time), count)| (string, time, count))
            .collect();

        let hash_set = counts
            .iter()
            .map(|(table, _, _)| table.clone())
            .collect::<std::collections::HashSet<String>>();

        for table in hash_set.iter() {
            make_table(table).await;
        }

        for (table, time, count) in counts.into_iter() {
            db::add_rec(table, time, count).await;
        }

        // 获取需要更新的记录
        let counts = context!()
            .counts
            .read()
            .iter()
            .filter(|(table, _)| hash_set.contains(*table))
            .map(|(table, count)| (table.clone(), *count))
            .collect::<Vec<(String, i64)>>();

        // TODO 原子化
        for (table, count) in counts.iter() {
            db::update_count(table, count).await;
        }
    }
}
