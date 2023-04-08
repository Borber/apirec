use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);

/// 配置
#[derive(Deserialize)]
pub struct ConfigFile {
    // 服务名称
    pub server_name: Option<String>,
    // 服务地址
    pub server_url: Option<String>,
    // 数据库目录
    pub database: Option<String>,
    // 日志目录
    pub log_dir: Option<String>,
    // 日志文件大小
    pub log_temp_size: Option<String>,
    // 日志级别
    pub log_level: Option<String>,
}

/// 配置
#[derive(Debug, Clone, Serialize)]
pub struct ApplicationConfig {
    // 服务名称
    pub server_name: String,
    // 服务地址
    pub server_url: String,
    // 数据库目录
    pub database: String,
    // 日志目录
    pub log_dir: String,
    // 日志文件大小
    pub log_temp_size: String,
    // 日志级别
    pub log_level: String,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = std::fs::read_to_string("application.yml").unwrap();
        let result: ConfigFile = serde_yaml::from_str(&yml_data).expect("load config file fail");
        let server_name = result
            .server_name
            .unwrap_or(env!("CARGO_PKG_NAME").to_owned());
        let server_url = result.server_url.unwrap_or("0.0.0.0:8000".to_owned());
        // TODO 优化相对位置
        let database = match result.database {
            Some(path) => path,
            None => {
                let exe_path = std::env::current_exe().expect("Failed to get current executable");
                let exe_path = exe_path.parent().unwrap();
                let file_path = exe_path.join("data").join("db.sqlite");
                file_path.to_str().unwrap().to_owned()
            }
        };
        let log_dir = result.log_dir.unwrap_or("log".to_owned());
        let log_temp_size = result.log_temp_size.unwrap_or("10mb".to_owned());
        let log_level = result.log_level.unwrap_or("info".to_owned());
        ApplicationConfig {
            server_name,
            server_url,
            database,
            log_dir,
            log_temp_size,
            log_level,
        }
    }
}
