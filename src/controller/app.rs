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
    if !context!().apps.check_app(&app) {
        return Resp::fail(APP_NOT_FOUND);
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
    if !util::is_valid(&app) {
        return Resp::fail(APP_NAME_IS_NO_VALID);
    }
    if context!().apps.check_app(&app) {
        return Resp::fail(APP_ALREADY_EXISTS);
    }

    info!("Add app: {}", app);

    // 将新增 app 添加到 apis 内存对象中优先提供计数功能,
    // 以保证 新增 api 时 app 存在
    //
    // Add the new app to the apis memory object to provide counting function first,
    // to ensure that the app exists when adding a new api
    context!().apis.add_app(&app);
    context!().apps.add(&app);
    context!().wait_app.add(&app);

    Resp::success("Add app success".to_owned())
}
