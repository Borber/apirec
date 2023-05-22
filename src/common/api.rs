use std::sync::atomic::{AtomicI64, Ordering};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;

/// 记录某app下所有api的调用次数
///
/// Record the number of calls to all apis under a certain app
pub type CountApi = Arc<RwLock<HashMap<String, Arc<AtomicI64>>>>;

/// 记录所有app的api调用次数
///
/// Record the number of calls to all apis of all apps
pub struct AllApi {
    map: Arc<RwLock<HashMap<String, CountApi>>>,
}

impl AllApi {
    pub fn new(map: HashMap<String, CountApi>) -> Self {
        Self {
            map: Arc::new(RwLock::new(map)),
        }
    }

    /// 将 api 的调用次数加一
    ///
    /// Add one to the number of calls to the api
    pub fn update(&self, app: &str, api: &str) -> i64 {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count = { count_api.read().get(api).unwrap().clone() };
        count.fetch_add(1, Ordering::SeqCst)
    }

    /// 添加一个 api
    ///
    /// Add a new api
    pub fn add_api(&self, app: &str, api: &str) {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        count_api
            .write()
            .insert(api.to_owned(), Arc::new(AtomicI64::new(0)));
    }

    /// 添加 app
    ///
    /// Add a new app
    pub fn add_app(&self, app: &str) {
        self.map
            .write()
            .insert(app.to_owned(), Arc::new(RwLock::new(HashMap::new())));
    }

    /// 获取 api 的调用次数
    ///
    /// Get the number of calls of the api
    pub fn get_api(&self, app: &str, api: &str) -> i64 {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count = { count_api.read().get(api).unwrap().clone() };
        count.load(Ordering::SeqCst)
    }

    /// 检测 api 是否存在
    ///
    /// Check if the api exists
    pub fn check_api(&self, app: &str, api: &str) -> bool {
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            return false;
        }
        self.map.read().get(app).unwrap().read().contains_key(api)
    }

    /// 获取 app 总调用次数
    ///
    /// Get the number of calls to all apis in the app
    pub fn get_sum(&self, app: &str) -> i64 {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count_api = count_api.read();

        count_api
            .values()
            .map(|count| count.load(Ordering::SeqCst))
            .sum()
    }

    /// 获取 app 的所有 api 的调用次数
    ///
    /// Get the number of calls to all apis in the app
    pub fn get_apis(&self, app: &str) -> HashMap<String, i64> {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count_api = count_api.read();

        count_api
            .iter()
            .map(|(api, count)| (api.to_owned(), count.load(Ordering::SeqCst)))
            .collect()
    }
}

/// 记录需要新增的 api
///
/// Record the api that needs to be added
type NewApi = Arc<RwLock<HashSet<String>>>;

/// 等待新增的api
///
/// The api that needs to be added
pub struct WaitApi {
    map: Arc<RwLock<HashMap<String, NewApi>>>,
}

impl WaitApi {
    pub fn new(map: HashMap<String, NewApi>) -> Self {
        Self {
            map: Arc::new(RwLock::new(map)),
        }
    }

    /// 添加一个 app
    ///
    /// Add a new app
    fn add_app(&self, app: &str) {
        self.map
            .write()
            .insert(app.to_owned(), Arc::new(RwLock::new(HashSet::new())));
    }

    /// 添加一个 api
    ///
    /// Add a new api
    pub fn add_api(&self, app: &str, api: &str) {
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            self.add_app(app);
        }
        let apis = { self.map.read().get(app).unwrap().clone() };
        apis.write().insert(api.to_owned());
    }

    /// 获取所有需要添加的 api
    ///
    /// Get all the apis that need to be added and clear the map
    pub fn get_apis(&self) -> HashMap<String, HashSet<String>> {
        let mut apps: HashMap<String, NewApi> = HashMap::new();
        std::mem::swap(&mut apps, &mut self.map.write());

        apps.into_iter()
            .map(|(app, apis)| {
                let mut apis_set = HashSet::new();
                std::mem::swap(&mut apis_set, &mut apis.write());
                (app, apis_set)
            })
            .collect()
    }
}
