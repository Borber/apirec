use axum::extract::Path;
use tracing::info;

use crate::{
    context,
    error::{APP_ALREADY_EXISTS, APP_NAME_IS_NO_VALID, APP_NOT_FOUND},
    handler::Json,
    model::{
        dto::{AddAppDTO, GetAppDTO},
        vo::app::{ApiCount, GetAppVO},
    },
    resp::Resp,
    util,
};

/// 获取 app 的访问量
///
/// Get app access count
pub async fn get(Path(app): Path<String>, body: Option<Json<GetAppDTO>>) -> Resp<GetAppVO> {
    // 检测 app 是否存在
    //
    // Check if app exists
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND.0, APP_NOT_FOUND.1);
    }
    match body {
        Some(Json(dto)) => {
            let apis = context!().apis.get_apis(&app);
            let total: i64 = apis.values().sum();

            // 将 apis 转换为 ApiCount 结构体, 以便排序
            //
            // Convert apis to ApiCount structure for sorting
            let mut apis: Vec<ApiCount> = apis
                .into_iter()
                .map(|(api, count)| ApiCount { api, count })
                .collect();

            // 除非特别指定, 否则默认按从大到小顺序
            //
            // Unless specified, the default is in descending order
            match dto.sort {
                Some(false) => apis.sort_by(|a, b| a.count.cmp(&b.count)),
                _ => apis.sort_by(|a, b| b.count.cmp(&a.count)),
            }

            // 返回部分结果
            //
            // Return part of the result
            let apis = match dto.apis {
                Some(parts) => apis
                    .into_iter()
                    .filter(|a| parts.contains(&a.api))
                    .collect(),
                None => match dto.limit {
                    Some(limit) => apis.into_iter().take(limit as usize).collect(),
                    None => apis,
                },
            };

            Resp::success(GetAppVO {
                total,
                apis: Some(apis),
            })
        }
        None => Resp::success(GetAppVO {
            total: context!().apis.get_sum(&app),
            apis: None,
        }),
    }
}

/// 新增 app
///
/// Add app
pub async fn add(Json(AddAppDTO { app }): Json<AddAppDTO>) -> Resp<String> {
    // 检测 app name 是否合法
    //
    // Check if app name is valid
    if !util::is_valid(&app) {
        return Resp::fail(APP_NAME_IS_NO_VALID.0, APP_NAME_IS_NO_VALID.1);
    };

    if context!().apps.check_app(&app) {
        return Resp::fail(APP_ALREADY_EXISTS.0, APP_ALREADY_EXISTS.1);
    }

    info!("Add app: {}", app);

    tokio::spawn(async move {
        // 新增 app
        //
        // Add app
        context!().apps.add(&app);
        // 将新增 app 添加到等待新增的 app 中
        //
        // Add the new app to the app to be added
        context!().wait_app.add(&app);
        // 将新增 app 添加到 apis 内存对象中优先提供计数功能, 并保证 新增 api 时 app 存在
        //
        // Add the new app to the apis memory object to provide the count function first, and ensure that the new api exists when the app exists
        context!().apis.add_app(&app);
    });

    Resp::success("Add app success".to_owned())
}
