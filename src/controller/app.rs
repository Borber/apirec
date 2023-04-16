use anyhow::Ok;
use axum::extract::Path;
use tracing::debug;

use crate::{
    context,
    handler::Json,
    model::{
        dto::AddAppDTO,
        vo::{app::GetAppVO, Resp, RespVO},
    },
    util,
};

// TODO 支持部分 api 的查询
// 获取 app 下所有 api的总访问量
// Get the total number of visits to all apis under the app
pub async fn get(Path(app): Path<String>) -> Resp<GetAppVO> {
    // 检测 app 是否存在
    // Check if app exists
    let flag = { context!().apps.read().get(&app).is_none() };
    if flag {
        return Json(RespVO::fail(1002, "App not found".to_owned()));
    }
    let apis = { context!().apis.read().get(&app).unwrap().clone() };
    let total = apis.values().sum::<i64>();
    Json(Ok(GetAppVO { total, apis }).into())
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
