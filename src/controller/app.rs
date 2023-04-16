use axum::{extract::Path, Json};
use tracing::debug;

use crate::{context, model::dto::AddAppDTO, util};

// TODO 返回总数和独立访问量
// 获取 app 下所有 api的总访问量
// Get the total number of visits to all apis under the app
pub async fn get(Path(app): Path<String>) -> String {
    let sum: i64 = context!()
        .apis
        .read()
        .get(&app)
        .unwrap()
        .iter()
        .map(|(_, v)| v)
        .sum();
    sum.to_string()
}

// 新增 app
// Add app
pub async fn add(Json(AddAppDTO { name }): Json<AddAppDTO>) -> String {
    // 检测 app name 是否合法
    // Check if app name is valid
    if !util::is_valid(&name) {
        return "Invalid app name".to_owned();
    };

    debug!("Add app: {}", name);

    let flag = { context!().apps.read().get(&name).is_some() };

    if flag {
        return "App already exists".to_owned();
    }

    {
        context!().apps.write().insert(name.clone());
    }

    {
        context!().wait_app.write().push(name);
    }

    "Success".to_owned()
}
