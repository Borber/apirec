use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddAppDTO {
    pub app: String,
}

#[derive(Deserialize, Debug)]
pub struct AddApiDTO {
    pub api: String,
}

/// 获取 app 的访问量
/// Get app access count
#[derive(Deserialize, Debug)]
pub struct GetAppDTO {
    /// 是否返回 app 全部 api 的访问量
    /// Whether to return the access count of all apis under the app
    pub all: Option<bool>,
    /// 是否需要排序
    /// Whether to sort
    pub sort: Option<bool>,
    /// 限制返回数量
    /// Limit the number of returns
    pub limit: Option<i64>,
    /// 指定 app 下的 api
    /// Specify the api under the app
    pub apis: Option<Vec<String>>,
}
