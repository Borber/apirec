use anyhow::Ok;
use axum::extract::Path;
use tracing::debug;

use crate::{
    context,
    handler::Json,
    model::{
        dto::AddAppDTO,
        vo::{Resp, RespVO},
    },
    util,
};

// TODO 返回总数和独立访问量
// 获取 app 下所有 api的总访问量
// Get the total number of visits to all apis under the app
pub async fn get(Path(app): Path<String>) -> Resp<i64> {
    let sum: i64 = context!()
        .apis
        .read()
        .get(&app)
        .unwrap()
        .iter()
        .map(|(_, v)| v)
        .sum();
    Json(Ok(sum).into())
}

// 新增 app
// Add app
pub async fn add(Json(AddAppDTO { name }): Json<AddAppDTO>) -> Resp<String> {
    // 检测 app name 是否合法
    // Check if app name is valid
    if !util::is_valid(&name) {
        return Json(RespVO::fail(1006, "App name is not valid".to_owned()));
    };

    debug!("Add app: {}", name);

    let flag = { context!().apps.read().get(&name).is_some() };

    if flag {
        return Json(RespVO::fail(1003, "App name is not valid".to_owned()));
    }

    {
        context!().apps.write().insert(name.clone());
    }

    {
        context!().wait_app.write().push(name);
    }

    Json(Ok("Success".to_owned()).into())
}
