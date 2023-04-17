use std::{collections::HashSet, sync::Arc};

use parking_lot::RwLock;

pub struct AllApp {
    pub set: Arc<RwLock<HashSet<String>>>,
}
impl AllApp {
    // 新增一个 app
    // Add a new app
    pub fn add(&self, app: &str) -> bool {
        self.set.write().insert(app.to_owned())
    }

    // 检测 app 是否存在
    // Check if the app exists
    pub fn check_app(&self, app: &str) -> bool {
        self.set.read().contains(app)
    }
}

pub struct WaitApp {
    set: Arc<RwLock<HashSet<String>>>,
}

impl WaitApp {
    pub fn new(set: HashSet<String>) -> Self {
        Self {
            set: Arc::new(RwLock::new(set)),
        }
    }
    // 新增一个 app
    // Add a new app
    pub fn add(&self, app: &str) -> bool {
        self.set.write().insert(app.to_owned())
    }
    // 获取所有需要新增的 App
    // Get all Apps that need to be added
    pub fn get_all(&self) -> HashSet<String> {
        // TODO 检查其他是否可以使用此方法优化
        self.set.write().drain().collect()
    }
}
