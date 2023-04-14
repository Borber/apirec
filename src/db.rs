use crate::{context, pool};

// 储存总调用次数
// TODO 优化为原子操作
pub async fn update_count(table: &str, count: &i64) {
    sqlx::query("update all_app set count = ? where name = ?")
        .bind(count)
        .bind(table)
        .execute(pool!())
        .await
        .unwrap();
}

// TODO 整理所有待添加的记录, 原子操作批量插入记录
pub async fn make_table(table: &str) {
    if context!().apps.read().get(table).is_none() {
        let sql = format!(
            r#"CREATE TABLE {} (
            "time" integer NOT NULL,
            "count" integer NOT NULL,
            PRIMARY KEY ("time")
        ); "#,
            table
        );

        // 新建表单
        sqlx::query(&sql).execute(pool!()).await.unwrap();

        // 将新建的表单插入到记录表中
        sqlx::query("insert into all_app (name, count) values (?, 0)")
            .bind(table)
            .execute(pool!())
            .await
            .unwrap();
        // 更新本地记录
        context!().apps.write().insert(table.to_owned());
    }
}

// TODO 新建单个 app 表

// TODO 新增 apps 记录

// TODO 原子操作批量插入记录
// TODO 修改返回类型为Result
pub async fn add_rec(table: String, time: i64, count: i32) -> bool {
    let sql = format!("insert into {} (time, count) values (?, ?)", table);
    match sqlx::query(&sql)
        .bind(time)
        .bind(count)
        .execute(pool!())
        .await
    {
        Ok(result) => result.rows_affected() == 1,
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}
