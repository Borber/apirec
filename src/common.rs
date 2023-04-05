use std::collections::{HashMap, HashSet};

use parking_lot::RwLock;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite};
use tokio::sync::OnceCell;

use crate::{config::CONFIG, model::AppRec};

pub static CONTEXT: OnceCell<ServiceContext> = OnceCell::const_new();

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::common::CONTEXT.get().unwrap().pool
    };
}

#[macro_export]
macro_rules! context {
    () => {
        $crate::common::CONTEXT.get().unwrap()
    };
}

pub async fn init() -> ServiceContext {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database_url)
        .await
        .unwrap();

    let counts: HashMap<String, i64> = sqlx::query_as("select name, count from all_table")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|rec: AppRec| (rec.name, rec.count))
        .collect();

    let dbmsg = vec![];

    let apps: HashSet<String> = sqlx::query("select name from all_table")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.get::<String, usize>(0))
        .collect();

    ServiceContext {
        apps: RwLock::new(apps),
        pool,
        counts: RwLock::new(counts),
        dbmsg: RwLock::new(dbmsg),
    }
}

pub struct ServiceContext {
    pub apps: RwLock<HashSet<String>>,
    pub pool: Pool<Sqlite>,
    pub counts: RwLock<HashMap<String, i64>>,
    pub dbmsg: RwLock<Vec<(String, i64)>>,
}
