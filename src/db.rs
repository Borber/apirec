use crate::pool;

// 更新app表中的api调用次数
// Update the number of api calls in the app table
pub async fn update_count(app: &str, api: &str, count: &i64) {
    let sql = format!("update {} set count = ? where api = ?", app);
    sqlx::query(&sql)
        .bind(count)
        .bind(api)
        .execute(pool!())
        .await
        .unwrap();
}

// 新增记录
// Add a record
pub async fn add_rec(app: &str, api: &str, time: &i64, count: &i64) {
    let sql = format!("insert into {}_{} (time, count) values (?, ?)", app, api);
    sqlx::query(&sql)
        .bind(time)
        .bind(count)
        .execute(pool!())
        .await
        .unwrap();
}

// 新建 api 表
// Make a new api table
pub async fn make_api_table(app: &str, api: &str) {
    let sql = format!(
        r#"CREATE TABLE {}_{} (
            "time" integer NOT NULL,
            "count" integer NOT NULL,
            PRIMARY KEY ("time")
        ); "#,
        app, api
    );

    // 新建表单
    // Make a new table
    sqlx::query(&sql).execute(pool!()).await.unwrap();

    // 将新建的表单插入到app表中
    // Insert the new table into the app table
    let sql = format!("insert into {} (api, count) values (?, 0)", app);
    sqlx::query(&sql).bind(api).execute(pool!()).await.unwrap();
}

// 新建 app 表
// Make a new app table
pub async fn make_app_table(app: &str) -> bool {
    let sql = format!(
        r#"CREATE TABLE {} (
        "api" text NOT NULL,
        "count" integer NOT NULL,
        PRIMARY KEY ("api")
    ); "#,
        app
    );

    // 新建表单
    sqlx::query(&sql).execute(pool!()).await.unwrap();

    // 将新建的表单插入到apps表中
    let sql = format!("insert into apps (app) values (?)");
    sqlx::query(&sql).bind(app).execute(pool!()).await.unwrap();

    true
}
