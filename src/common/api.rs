use std::sync::atomic::{AtomicI64, Ordering};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;

use crate::model::vo::app::GetAppVO;

/// 记录某app下所有api的调用次数
/// Record the number of calls to all apis under a certain app
type CountApi = Arc<RwLock<HashMap<String, Arc<AtomicI64>>>>;

/// 记录所有app的api调用次数
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
    /// Add one to the number of calls to the api
    pub fn update(&self, app: &str, api: &str) -> i64 {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count = { count_api.read().get(api).unwrap().clone() };
        count.fetch_add(1, Ordering::SeqCst)
    }

    /// 添加一个 api
    /// Add a new api
    pub fn add_api(&self, app: &str, api: &str) {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let mut count_api = count_api.write();
        count_api.insert(api.to_owned(), Arc::new(AtomicI64::new(0)));
    }

    /// 添加 app
    /// Add a new app
    pub fn add_app(&self, app: &str) {
        let mut apps = self.map.write();
        apps.insert(app.to_owned(), Arc::new(RwLock::new(HashMap::new())));
    }

    /// 获取 api 的调用次数
    /// Get the number of calls to the api
    pub fn get_api(&self, app: &str, api: &str) -> i64 {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count = { count_api.read().get(api).unwrap().clone() };
        count.load(Ordering::SeqCst)
    }

    /// 检测 api 是否存在
    /// Check if the app and api exist
    pub fn check_api(&self, app: &str, api: &str) -> bool {
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            return false;
        }
        self.map.read().get(app).unwrap().read().contains_key(api)
    }

    /// 获取 app 的所有 api 的调用次数
    /// Get the number of calls to all apis in the app
    pub fn get_apis(&self, app: &str) -> GetAppVO {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let mut apis = HashMap::new();
        {
            let count_api = count_api.read();

            for (api, count) in count_api.iter() {
                apis.insert(api.to_owned(), count.load(Ordering::SeqCst));
            }
        }
        let total = apis.values().sum();
        GetAppVO { total, apis }
    }
}

/// 记录需要新增的 api
/// Record the api that needs to be added
type NewApi = Arc<RwLock<HashSet<String>>>;

/// 等待新增的api
/// Waiting for the new api to be added
pub struct WaitApi {
    map: Arc<RwLock<HashMap<String, NewApi>>>,
}

impl WaitApi {
    pub fn new(map: HashMap<String, NewApi>) -> Self {
        Self {
            map: Arc::new(RwLock::new(map)),
        }
    }
    /// 添加一个 api
    /// Add a new api
    pub fn add_api(&self, app: &str, api: &str) {
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            self.add_app(app);
        }
        let apis = { self.map.read().get(app).unwrap().clone() };
        let mut apis = apis.write();
        apis.insert(api.to_owned());
    }

    /// 添加一个 app
    /// Add a new app
    fn add_app(&self, app: &str) {
        let mut apps = self.map.write();
        apps.insert(app.to_owned(), Arc::new(RwLock::new(HashSet::new())));
    }

    // TODO 检查其他是否可以使用此方法优化
    // self.set.write().drain().collect()
    /// 获取所有需要添加的 api 并随后清空 map
    /// Get all the apis that need to be added and then clear the map
    pub fn get_apis(&self) -> HashMap<String, HashSet<String>> {
        let mut map = HashMap::new();
        {
            let apps = { self.map.read() };
            for (app, apis) in apps.iter() {
                let apis = { apis.read().clone() };
                map.insert(app.to_owned(), apis);
            }
        }
        {
            let mut lock = self.map.write();
            lock.clear();
        }
        map
    }
}
