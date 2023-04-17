pub mod api;
pub mod app;
pub mod record;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::sync::OnceCell;

use crate::{
    common::app::AllApp,
    config::CONFIG,
    model::{Api, App},
};

use self::{
    api::{AllApi, WaitApi},
    app::WaitApp,
    record::WaitRecord,
};

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
    let file_path_s = file_path.to_str().unwrap().to_owned();
    let file_path_s = file_path_s.replace("\\\\?\\", "");
    let db_path = format!("sqlite://{}", file_path_s);
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
            CREATE TABLE "apps" (
                "app" TEXT NOT NULL,
                PRIMARY KEY ("app")
            );
        "#;
        sqlx::query(sql).execute(&conn).await.unwrap();
    }

    // 创建数据库连接池
    // Create the database pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await
        .unwrap();

    // 获取所有 app
    // Get all apps
    let apps: HashSet<String> = sqlx::query_as("select app from apps")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|item: App| item.app)
        .collect();

    println!("init apps: {:?}", apps);

    // 获取每个app中各api的调用次数
    // Get the number of calls to each api in each app
    let mut apis = HashMap::new();
    for app in &apps {
        let sql = format!("select api, count from {}", app);

        let apis_part: HashMap<String, Arc<RwLock<i64>>> = sqlx::query_as(&sql)
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|api: Api| (api.api, Arc::new(RwLock::new(api.count))))
            .collect();

        apis.insert(app.to_owned(), Arc::new(RwLock::new(apis_part)));
    }
    println!("init apis: {:?}", apis);

    ServiceContext {
        apps: AllApp {
            set: Arc::new(RwLock::new(apps)),
        },
        pool,
        apis: AllApi::new(apis),
        wait_app: WaitApp::new(HashSet::new()),
        wait_api: WaitApi::new(HashMap::new()),
        wait_record: WaitRecord::new(HashMap::new()),
    }
}

pub struct ServiceContext {
    // 记录所有已经存在的app
    // Record all existing apps
    pub apps: AllApp,

    // 数据库连接池
    // Database connection pool
    pub pool: Pool<Sqlite>,

    // 记录总调用次数
    // Record the total number of calls
    pub apis: AllApi,

    // 等待新增的app
    // Waiting for the new app to be added
    pub wait_app: WaitApp,

    // 等待新增的api
    // Waiting for the new api to be added
    pub wait_api: WaitApi,

    // 等待新增的记录
    // Waiting for new records to be added
    pub wait_record: WaitRecord,
}
