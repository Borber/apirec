use hashbrown::HashSet;
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
///
/// Get app access count
#[derive(Deserialize, Debug)]
pub struct GetAppDTO {
    /// 排序方式指定
    ///
    /// Sort method specification
    pub sort: Option<bool>,
    /// 限制返回数量
    ///
    /// Limit the number of returns
    pub limit: Option<i64>,
    /// 指定 app 下的 api
    ///
    /// Specify the api under the app
    pub apis: Option<HashSet<String>>,
}
