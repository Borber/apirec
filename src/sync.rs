use std::{collections::HashMap, time::Duration};

use tracing::info;

use crate::{
    config::CONFIG,
    context,
    db::{add_rec, make_api_table, make_app_table, update_count},
};

/// 数据库同步
/// Database sync
pub async fn db_sync() {
    info!("Database sync task started");
    loop {
        tokio::time::sleep(Duration::from_secs(CONFIG.sync_interval)).await;

        // 获取需要新增的 app
        // Get new app
        let wait_app = { context!().wait_app.get_all() };

        // 添加 app, 建立相关表
        // Add app, build related tables
        if !wait_app.is_empty() {
            info!("wait_app: {:?}", wait_app);
            for app in wait_app.iter() {
                make_app_table(app).await;
            }
        }

        // 获取需要新增的api
        // Get new api
        let wait_api = { context!().wait_api.get_apis() };

        // 添加 api, 建立相关表
        // Add api, build related tables
        if !wait_api.is_empty() {
            info!("wait_api: {:?}", wait_api);
            for (app, apis) in wait_api.iter() {
                for api in apis.iter() {
                    make_api_table(app, api).await;
                }
            }
        }

        // TODO 直接清空列表, 随后拿到返回的数据再进行处理, 随后直接清空
        // 获取需要新增的记录
        // Get new record
        let wait_record = { context!().wait_record.get_records() };
        if !wait_record.is_empty() {
            info!("wait_record: {:?}", wait_record);

            // TODO 一次性拿取所有值, 仅加一次写锁
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

            info!("api_update: {:?}", api_update);

            // 更新api表中的记录
            // Update the record in the api table
            for (app, apis) in api_update.iter() {
                for (api, count) in apis.iter() {
                    update_count(app, api, count).await;
                }
            }

            // 添加记录
            // Add record
            for (app, apis) in wait_record.iter() {
                for (api, times) in apis.iter() {
                    for (time, count) in times.iter() {
                        add_rec(app, api, time, count).await;
                    }
                }
            }
        }
    }
}
