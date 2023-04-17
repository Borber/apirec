use std::{collections::HashMap, time::Duration};

use tracing::{debug, info};

use crate::{
    context,
    db::{add_rec, make_api_table, make_app_table, update_count},
};

// TODO 支持设置同步间隔
// TODO 尽量短的写锁, 尽量长的读锁 使用读锁来复制数据, 使用写锁来更新数据
// 数据库同步
pub async fn db_sync() {
    info!("Database sync started");
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;

        // 获取需要新增的 app
        // Get new app
        let wait_app = { context!().wait_app.get_all() };

        debug!("wait_app: {:?}", wait_app);

        // 新增 app
        // Add app
        for app in wait_app.iter() {
            make_app_table(app).await;
        }

        // 获取需要新增的api
        // Get new api
        let wait_api = context!().wait_api.get_apis();

        // 新增 api
        // Add api
        for (app, apis) in wait_api.iter() {
            for api in apis.iter() {
                make_api_table(app, api).await;
            }
        }

        // 获取需要新增的记录
        // Get new record
        let wait_record = context!().wait_record.get_records();

        // TODO 一次性拿去所有值, 仅加一次读锁
        // 需要更新Api的值
        // Api value to be updated
        let api_update: HashMap<&String, HashMap<&String, i64>> = wait_record
            .iter()
            .map(|(app, apis)| {
                let apis: HashMap<&String, i64> = apis
                    .iter()
                    .map(|(api, _)| (api, context!().apis.get_api(app, api)))
                    .collect();
                (app, apis)
            })
            .collect();

        // 更新api表
        for (app, apis) in api_update.iter() {
            for (api, count) in apis.iter() {
                update_count(app, api, count).await;
            }
        }

        // 更新记录表
        for (app, apis) in wait_record.iter() {
            for (api, times) in apis.iter() {
                for (time, count) in times.iter() {
                    add_rec(app, api, time, count).await;
                }
            }
        }
    }
}
