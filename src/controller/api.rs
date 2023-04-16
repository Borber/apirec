use std::{
    collections::{HashMap, HashSet},
    time::UNIX_EPOCH,
};

use axum::{extract::Path, Json};
use tracing::info;

use crate::{context, model::dto::AddApiDTO, util};

pub async fn add(Path(app): Path<String>, Json(AddApiDTO { api }): Json<AddApiDTO>) -> String {
    // 检测 api name 是否合法
    // Check if api name is valid
    if !util::is_valid(&api) {
        return "Invalid api name".to_owned();
    };

    // 检测 app 是否存在
    // Check if app exists
    let flag = { context!().apps.read().get(&app).is_none() };
    if flag {
        return "App not found".to_owned();
    }

    // 检测 api 是否已经存在
    let flag = {
        context!()
            .apis
            .read()
            .get(&app)
            .unwrap()
            .get(&api)
            .is_some()
    };

    if flag {
        return "Api already exists".to_owned();
    }

    info!("Add api: {} to app: {}", api, app);

    // 将新增 api 添加到 apis内存对象中优先提供计数功能
    // Add the new api to the apis memory object to provide the count function first
    {
        context!()
            .apis
            .write()
            .entry(app.clone())
            .or_insert_with(HashMap::new)
            .insert(api.clone(), 0);
    }

    // 将新增api添加到 wait api 中
    // Add the new api to wait api
    {
        context!()
            .wait_api
            .write()
            .entry(app)
            .or_insert_with(HashSet::new)
            .insert(api);
    }
    "Success".to_owned()
}

// 获取 api 访问数量
// Get api access count
pub async fn get(Path((app, api)): Path<(String, String)>) -> String {
    match context!().apis.read().get(&app) {
        Some(apis) => apis
            .get(&api)
            .map(|e| e.to_string())
            .unwrap_or_else(|| "Api not found".to_owned()),
        None => "App not found".to_owned(),
    }
}

// 新增记录
// Add record
pub async fn post(Path((app, api)): Path<(String, String)>) -> String {
    let flag = { context!().apps.read().get(&app).is_none() };
    if flag {
        return "App not found".to_owned();
    };

    let flag = {
        context!()
            .apis
            .read()
            .get(&app)
            .unwrap()
            .get(&api)
            .is_none()
    };
    if flag {
        return "Api not found".to_owned();
    };

    // 新增内存中的 api 访问数
    // Update api access count in memory
    {
        context!()
            .apis
            .write()
            .entry(app.clone())
            .or_insert_with(HashMap::new)
            .entry(api.clone())
            .and_modify(|e| *e += 1);
    }

    // 获取时间戳
    // Get timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // 新增记录中的时间戳
    // Update timestamp in record
    {
        context!()
            .wait_record
            .write()
            .entry(app)
            .or_insert_with(HashMap::new)
            .entry(api)
            .or_insert_with(HashMap::new)
            .entry(timestamp)
            .and_modify(|e| *e += 1)
            .or_insert_with(|| 1);
    }

    "Success".to_owned()
}
