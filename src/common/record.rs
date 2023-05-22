use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
    time::UNIX_EPOCH,
};

use parking_lot::RwLock;

type Record = Arc<RwLock<HashMap<i64, Arc<AtomicI64>>>>;

type RecordApi = Arc<RwLock<HashMap<String, Record>>>;

/// 等待新增的记录
///
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
    ///
    /// Add record
    pub fn add(&self, app: &str, api: &str) {
        // 若 app 不存在, 则添加 app
        // If the app does not exist, add the app
        let flag = { self.map.read().contains_key(app) };
        if !flag {
            self.map
                .write()
                .insert(app.to_owned(), Arc::new(RwLock::new(HashMap::new())));
        }
        // 若 api 不存在, 则添加 api
        // If the api does not exist, add the api
        let record_api = { self.map.read().get(app).unwrap().clone() };
        let flag = { record_api.read().contains_key(api) };
        if !flag {
            record_api
                .write()
                .insert(api.to_owned(), Arc::new(RwLock::new(HashMap::new())));
        }

        let time = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let app = { self.map.read().get(app).unwrap().clone() };
        let api = { app.read().get(api).unwrap().clone() };
        api.write()
            .entry(time)
            .and_modify(|e| {
                e.fetch_add(1, Ordering::SeqCst);
            })
            .or_insert(Arc::new(AtomicI64::new(1)));
    }

    /// 获取所有需要添加的记录并清空 map
    ///
    /// Get all records that need to be added and clear the map
    pub fn get_records(&self) -> HashMap<String, HashMap<String, HashMap<i64, i64>>> {
        let mut map = HashMap::new();
        let mut apps: HashMap<String, RecordApi> = HashMap::new();
        std::mem::swap(&mut apps, &mut self.map.write());
        for (app, app_record) in apps.into_iter() {
            let mut apis = HashMap::new();
            let mut app_record_map: HashMap<String, Record> = HashMap::new();
            std::mem::swap(&mut app_record_map, &mut app_record.write());
            for (api, api_record) in app_record_map {
                let mut times = HashMap::new();
                let mut api_record_map: HashMap<i64, Arc<AtomicI64>> = HashMap::new();
                std::mem::swap(&mut api_record_map, &mut api_record.write());
                for (time, count) in api_record_map {
                    times.insert(time, count.load(Ordering::SeqCst));
                }
                apis.insert(api, times);
            }
            map.insert(app, apis);
        }
        map
    }
}
