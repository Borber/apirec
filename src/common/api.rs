use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;
use tracing::debug;

use crate::model::vo::app::GetAppVO;

type CountApi = Arc<RwLock<HashMap<String, Arc<RwLock<i64>>>>>;

// 记录总调用次数
// Record the total number of calls
pub struct AllApi {
    map: Arc<RwLock<HashMap<String, CountApi>>>,
}

impl AllApi {
    pub fn new(map: HashMap<String, CountApi>) -> Self {
        Self {
            map: Arc::new(RwLock::new(map)),
        }
    }

    // 将 api 的调用次数加一
    // Add one to the number of calls to the api
    pub fn update(&self, app: &str, api: &str) {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count = { count_api.read().get(api).unwrap().clone() };
        let mut count = count.write();
        *count += 1;
    }

    // TODO 若新增接口处有提前检测, 则无需再次检测是否已存在
    // 新增一个 api
    // Add a new api
    pub fn add_api(&self, app: &str, api: &str) {
        let flag = { self.map.read().contains_key(api) };
        if !flag {
            self.add_app(app);
        }
        let flag = { self.map.read().get(app).unwrap().read().contains_key(api) };
        if flag {
            return;
        }
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let mut count_api = count_api.write();
        count_api.insert(api.to_owned(), Arc::new(RwLock::new(0)));
    }

    // 新增一个 app
    // Add a new app
    fn add_app(&self, app: &str) {
        let mut apps = self.map.write();
        apps.insert(app.to_owned(), Arc::new(RwLock::new(HashMap::new())));
    }

    // 获取 api 的调用次数
    // Get the number of calls to the api
    pub fn get_api(&self, app: &str, api: &str) -> i64 {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count = { count_api.read().get(api).unwrap().clone() };
        let count = count.read();
        *count
    }

    // 检测 api 是否存在
    // Check if the app and api exist
    pub fn check_api(&self, app: &str, api: &str) -> bool {
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            return false;
        }
        self.map.read().get(app).unwrap().read().contains_key(api)
    }

    // 获取 app 的所有 api 的调用次数
    // Get the number of calls to all apis in the app
    pub fn get_apis(&self, app: &str) -> GetAppVO {
        let count_api = { self.map.read().get(app).unwrap().clone() };
        let count_api = count_api.read();
        let mut apis = HashMap::new();
        for (api, count) in count_api.iter() {
            apis.insert(api.to_owned(), *count.read());
        }
        let total = apis.values().sum();
        GetAppVO { total, apis }
    }
}

// 记录需要新增的 api
// Record the api that needs to be added
type NewApi = Arc<RwLock<HashSet<String>>>;

// 等待新增的api
// Waiting for the new api to be added
pub struct WaitApi {
    map: Arc<RwLock<HashMap<String, NewApi>>>,
}

impl WaitApi {
    pub fn new(map: HashMap<String, NewApi>) -> Self {
        Self {
            map: Arc::new(RwLock::new(map)),
        }
    }
    // 新增一个 api
    // Add a new api
    pub fn add_api(&self, app: &str, api: &str) {
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            self.add_app(app);
        }
        let apis = { self.map.read().get(app).unwrap().clone() };
        let mut apis = apis.write();
        apis.insert(api.to_owned());
    }

    // 新增一个 app
    // Add a new app
    fn add_app(&self, app: &str) {
        let mut apps = self.map.write();
        apps.insert(app.to_owned(), Arc::new(RwLock::new(HashSet::new())));
    }

    // TODO 检查其他是否可以使用此方法优化
    // self.set.write().drain().collect()
    // 获取所有需要添加的 api 并随后清空 map
    // Get all the apis that need to be added and then clear the map
    pub fn get_apis(&self) -> HashMap<String, HashSet<String>> {
        let mut map = HashMap::new();
        {
            let apps = { self.map.read() };
            for (app, apis) in apps.iter() {
                let apis = { apis.read().clone() };
                map.insert(app.to_owned(), apis);
            }
        }
        debug!("get_apis: {:?}", map);
        self.map.write().clear();
        debug!("wait_api clear");
        map
    }
}
