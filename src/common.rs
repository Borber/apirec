use std::collections::{HashMap, HashSet};

use parking_lot::RwLock;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
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
    let file_path = CONFIG.exe_dir.join("data").join("db.sqlite");
    let db_path = format!("sqlite://{}", file_path.to_str().unwrap().to_owned());
    // 检测数据库是否存在
    if !file_path.exists() {
        // 如果数据库文件不存在，创建数据库文件
        // Create the data directory if it doesn't exist
        std::fs::create_dir(file_path.parent().unwrap()).unwrap();
        std::fs::File::create(file_path).unwrap();
        // 创建数据库
        // Connect to the database
        let conn = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_path)
            .await
            .unwrap();
        let sql = r#"
            DROP TABLE IF EXISTS "all_app";
            CREATE TABLE "all_app" (
                "name" TEXT NOT NULL,
                "count" integer NOT NULL,
                PRIMARY KEY ("name")
            );"#;
        sqlx::query(sql).execute(&conn).await.unwrap();
    }
    // 创建数据库连接池
    // Create the database pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await
        .unwrap();

    // 读取数据库中存储的记录
    // Read the records from the database
    let counts: HashMap<String, i64> = sqlx::query_as("select name, count from all_app")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|rec: AppRec| (rec.name, rec.count))
        .collect();

    let apps: HashSet<String> = counts.keys().map(|s| s.to_owned()).collect();

    let wait_record = vec![];
    let wait_app = vec![];

    ServiceContext {
        apps: RwLock::new(apps),
        pool,
        counts: RwLock::new(counts),
        wait_record: RwLock::new(wait_record),
        wait_app: RwLock::new(wait_app),
    }
}

pub struct ServiceContext {
    // 记录所有已经存在的app
    // Record all existing apps
    pub apps: RwLock<HashSet<String>>,

    // 数据库连接池
    // Database connection pool
    pub pool: Pool<Sqlite>,

    // 记录总调用次数
    // Record the total number of calls
    pub counts: RwLock<HashMap<String, i64>>,

    // 等待新增的记录
    // Waiting for new records to be added
    pub wait_record: RwLock<Vec<(String, i64)>>,

    // 等待新增的app
    // Waiting for the new app to be added
    pub wait_app: RwLock<Vec<String>>,
}
