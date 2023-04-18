use anyhow::Ok;
use axum::extract::Path;
use tracing::info;

use crate::{
    context,
    handler::Json,
    model::{
        dto::{AddAppDTO, GetAppDTO},
        vo::{app::GetAppVO, Resp, RespVO},
    },
    util,
};

// TODO 支持部分 api 的查询
// TODO 支持排序
// TODO 数量限制
/// 获取 app 的访问量
/// Get app access count
pub async fn get(Path(app): Path<String>, body: Option<Json<GetAppDTO>>) -> Resp<GetAppVO> {
    // 检测 app 是否存在
    // Check if app exists
    let flag = { context!().apps.check_app(&app) };
    if !flag {
        return Json(RespVO::fail(1002, "App not found".to_owned()));
    }
    match body {
        Some(Json(dto)) => {
            println!("dto:{:?}", dto);
            Json(Ok(context!().apis.get_apis(&app)).into())
        }
        None => {
            println!("None");
            Json(Ok(context!().apis.get_apis(&app)).into())
        }
    }
}

/// 新增 app
/// Add app
pub async fn add(Json(AddAppDTO { app }): Json<AddAppDTO>) -> Resp<String> {
    // 检测 app name 是否合法
    // Check if app name is valid
    if !util::is_valid(&app) {
        return Json(RespVO::fail(1006, "App name is not valid".to_owned()));
    };

    let flag = { context!().apps.check_app(&app) };

    if flag {
        return Json(RespVO::fail(1003, "App already exists".to_owned()));
    }

    info!("Add app: {}", app);

    {
        context!().apps.add(&app);
    }
    {
        context!().wait_app.add(&app);
    }
    // 将新增 app 添加到 apis 内存对象中优先提供计数功能, 并保证 新增 api 时 app 存在
    // Add the new app to the apis memory object to provide the count function first, and ensure that the new api exists when the app exists
    {
        context!().apis.add_app(&app);
    }

    Json(Ok("Success".to_owned()).into())
}
