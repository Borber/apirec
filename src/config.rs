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
    // 日志级别
    pub log_level: Option<String>,
    // 日志分割
    pub log_split: Option<String>,
}

/// 配置
#[derive(Debug, Clone, Serialize)]
pub struct ApplicationConfig {
    // 服务名称
    pub server_name: String,
    // 服务地址
    pub server_url: String,
    // 日志级别
    pub log_level: String,
    // 日志分割
    pub log_split: String,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let config_data = std::fs::read_to_string("config.toml").unwrap();
        let result: ConfigFile = toml::from_str(&config_data).expect("load config file fail");
        let server_name = result
            .server_name
            .unwrap_or(env!("CARGO_PKG_NAME").to_owned());
        let server_url = result.server_url.unwrap_or("0.0.0.0:8000".to_owned());
        let log_level = result.log_level.unwrap_or("info".to_owned());
        let log_split = result.log_split.unwrap_or("day".to_owned());
        ApplicationConfig {
            server_name,
            server_url,
            log_level,
            log_split,
        }
    }
}
