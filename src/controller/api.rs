use anyhow::Ok;
use axum::extract::Path;
use tracing::info;

use crate::{
    context,
    handler::Json,
    model::{
        dto::AddApiDTO,
        vo::{Resp, RespVO},
    },
    util,
};

/// 新增 Api
///
/// Add Api
pub async fn add(
    Path(app): Path<String>,
    Json(AddApiDTO { api }): Json<AddApiDTO>,
) -> Resp<String> {
    // 检测 api name 是否合法
    // Check if api name is valid
    if !util::is_valid(&api) {
        return Json(RespVO::fail(1001, "Api name is not valid".to_owned()));
    };

    // 检测 app 是否存在
    // Check if app exists
    if !context!().apps.check_app(&app) {
        return Json(RespVO::fail(1002, "App not found".to_owned()));
    }

    // 检测 api 是否已经存在
    // Check if api already exists
    if context!().apis.check_api(&app, &api) {
        return Json(RespVO::fail(1003, "Api already exists".to_owned()));
    }

    info!("Add api: {} to app: {}", api, app);

    tokio::spawn(async move {
        // 将新增 api 添加到 apis内存对象中优先提供计数功能
        // 此处保证 api 存在时 app 必定存在 即, 当 app 不存在时 api 一定不存在
        // Add the new api to the apis memory object to provide the count function first
        // Here to ensure that when the api exists, the app must exist, that is, when the app does not exist, the api must not exist
        context!().apis.add_api(&app, &api);

        // 将新增api添加到 wait api 中
        // Add the new api to wait api
        context!().wait_api.add_api(&app, &api);
    });

    Json(RespVO::success("Success".to_owned()))
}

/// 获取 api 访问数量
///
/// Get api access count
pub async fn get(Path((app, api)): Path<(String, String)>) -> Resp<i64> {
    // 检测 app 是否存在
    // Check if app exists
    if !context!().apps.check_app(&app) {
        return Json(RespVO::fail(1002, "App not found".to_owned()));
    };
    // 检测 api 是否存在
    // Check if api exists
    if !context!().apis.check_api(&app, &api) {
        return Json(RespVO::fail(1004, "Api not found".to_owned()));
    };
    Json(Ok(context!().apis.get_api(&app, &api)).into())
}

/// 新增记录
///
/// Add record
pub async fn post(Path((app, api)): Path<(String, String)>) -> Resp<i64> {
    // 检测 app 是否存在
    // Check if app exists
    if !context!().apps.check_app(&app) {
        return Json(RespVO::fail(1002, "App not found".to_owned()));
    };

    // 检测 api 是否存在
    // Check if api exists
    if !context!().apis.check_api(&app, &api) {
        return Json(RespVO::fail(1004, "Api not found".to_owned()));
    };

    let count = context!().apis.get_api(&app, &api) + 1;

    tokio::spawn(async move {
        // 新增内存中的 api 访问数
        // Update api access count in memory
        context!().apis.update(&app, &api);

        // 新增记录
        // Add record
        context!().wait_record.add(&app, &api);
    });

    Json(RespVO::success(count))
}
