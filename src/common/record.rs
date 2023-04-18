use std::{collections::HashMap, sync::Arc, time::UNIX_EPOCH};

use parking_lot::RwLock;

type RecordApi = Arc<RwLock<HashMap<String, Arc<RwLock<HashMap<i64, Arc<RwLock<i64>>>>>>>>;

/// 等待新增的记录
/// Waiting for new records to be added
pub struct WaitRecord {
    map: Arc<RwLock<HashMap<String, RecordApi>>>,
}

impl WaitRecord {
    pub fn new(map: HashMap<String, RecordApi>) -> Self {
        Self {
            map: Arc::new(RwLock::new(map)),
        }
    }

    /// 添加记录
    /// Add record
    pub fn add(&self, app: &str, api: &str) {
        // 若 app 不存在, 则添加 app
        // If the app does not exist, add the app
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            let mut lock = self.map.write();
            lock.insert(app.to_owned(), Arc::new(RwLock::new(HashMap::new())));
        }
        // 若 api 不存在, 则添加 api
        // If the api does not exist, add the api
        let record_api = { self.map.read().get(app).unwrap().clone() };
        let flag = { record_api.read().contains_key(api) };
        if !flag {
            let mut lock = record_api.write();
            lock.insert(api.to_owned(), Arc::new(RwLock::new(HashMap::new())));
        }

        let time = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let app = { self.map.read().get(app).unwrap().clone() };
        let api = { app.read().get(api).unwrap().clone() };
        let mut lock = api.write();
        lock.entry(time)
            .and_modify(|e| {
                let mut e = e.write();
                *e += 1;
            })
            .or_insert(Arc::new(RwLock::new(1)));
    }

    // TODO 检查其他是否可以使用此方法优化
    // TODO 是否应该先全部move出, 再重新构建
    // self.set.write().drain().collect()
    /// 获取所有需要添加的记录并随后清空 map
    /// Get all the records that need to be added and then clear the map
    pub fn get_records(&self) -> HashMap<String, HashMap<String, HashMap<i64, i64>>> {
        let mut map = HashMap::new();
        {
            let apps = self.map.read();
            for (app, record_api) in apps.iter() {
                let mut apis = HashMap::new();
                let record_api = record_api.read();
                for (api, record) in record_api.iter() {
                    let mut times = HashMap::new();
                    let record = record.read();
                    for (time, count) in record.iter() {
                        times.insert(*time, *count.read());
                    }
                    apis.insert(api.to_owned(), times);
                }
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
