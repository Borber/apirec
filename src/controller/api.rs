use anyhow::Ok;
use axum::extract::Path;
use tracing::info;

use crate::{
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
    if !util::is_valid(&api) {
        return Resp::fail(API_NAME_IS_NO_VALID);
    }
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND);
    }
    if context!().apis.check_api(&app, &api) {
        return Resp::fail(API_ALREADY_EXISTS);
    }

    info!("Add api: {} to app: {}", api, app);

    context!().apis.add_api(&app, &api);
    context!().wait_api.add_api(&app, &api);

    Resp::success("Success".to_owned())
}

/// 获取 api 访问数量
///
/// Get api access count
pub async fn get(Path((app, api)): Path<(String, String)>) -> Resp<i64> {
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND);
    };
    if !context!().apis.check_api(&app, &api) {
        return Resp::fail(API_NOT_FOUND);
    };
    Ok(context!().apis.get_api(&app, &api)).into()
}

/// 新增记录
///
/// Add record
pub async fn post(Path((app, api)): Path<(String, String)>) -> Resp<i64> {
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND);
    };
    if !context!().apis.check_api(&app, &api) {
        return Resp::fail(API_NOT_FOUND);
    };

    let count = context!().apis.get_api(&app, &api) + 1;

    context!().apis.update(&app, &api);
    context!().wait_record.add(&app, &api);

    Resp::success(count)
}
