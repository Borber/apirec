use std::sync::Arc;

use anyhow::Ok;
use axum::{extract::Path, Extension};
use tracing::info;

use crate::{
    common::ServiceContext,
    context,
    error::{API_ALREADY_EXISTS, API_NAME_IS_NO_VALID, API_NOT_FOUND, APP_NOT_FOUND},
    handler::Json,
    model::dto::AddApiDTO,
    resp::Resp,
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
    //
    // Check if api name is valid
    if !util::is_valid(&api) {
        return Resp::fail(API_NAME_IS_NO_VALID.0, API_NAME_IS_NO_VALID.1);
    };

    // 检测 app 是否存在
    //
    // Check if app exists
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND.0, APP_NOT_FOUND.1);
    }

    // 检测 api 是否已经存在
    //
    // Check if api already exists
    if context!().apis.check_api(&app, &api) {
        return Resp::fail(API_ALREADY_EXISTS.0, API_ALREADY_EXISTS.1);
    }

    info!("Add api: {} to app: {}", api, app);

    // 将新增 api 添加到 apis内存对象中优先提供计数功能
    // 此处保证 api 存在时 app 必定存在 即, 当 app 不存在时 api 一定不存在
    //
    // Add the new api to the apis memory object to provide the count function first
    // Here to ensure that when the api exists, the app must exist, that is, when the app does not exist, the api must not exist
    context!().apis.add_api(&app, &api);

    // 将新增api添加到 wait api 中
    //
    // Add the new api to wait api
    context!().wait_api.add_api(&app, &api);

    Resp::success("Success".to_owned())
}

/// 获取 api 访问数量
///
/// Get api access count
pub async fn get(Path((app, api)): Path<(String, String)>) -> Resp<i64> {
    // 检测 app 是否存在
    //
    // Check if app exists
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND.0, APP_NOT_FOUND.1);
    };
    // 检测 api 是否存在
    //
    // Check if api exists
    if !context!().apis.check_api(&app, &api) {
        return Resp::fail(APP_NOT_FOUND.0, APP_NOT_FOUND.1);
    };
    Ok(context!().apis.get_api(&app, &api)).into()
}

/// 新增记录
///
/// Add record
pub async fn post(Path((app, api)): Path<(String, String)>) -> Resp<i64> {
    // 检测 app 是否存在
    //
    // Check if app exists
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND.0, APP_NOT_FOUND.1);
    };

    // 检测 api 是否存在
    //
    // Check if api exists
    if !context!().apis.check_api(&app, &api) {
        return Resp::fail(API_NOT_FOUND.0, API_NOT_FOUND.1);
    };

    let count = context!().apis.get_api(&app, &api) + 1;

    // 新增内存中的 api 访问数
    //
    // Update api access count in memory
    context!().apis.update(&app, &api);

    // 新增记录
    //
    // Add record
    context!().wait_record.add(&app, &api);

    Resp::success(count)
}
