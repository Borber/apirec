use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);

/// 配置
/// Configuration
#[derive(Deserialize)]
pub struct ConfigFile {
    /// 服务名称
    /// Service name
    pub server_name: Option<String>,
    /// 服务端口
    /// Service port
    pub port: Option<u16>,
    /// 日志级别
    /// Log level
    pub log_level: Option<String>,
    /// 日志分割
    /// Log split
    pub log_split: Option<String>,
    /// 同步间隔
    /// Synchronization interval
    pub sync_interval: Option<u64>,
}

/// 配置
/// Configuration
#[derive(Debug, Clone, Serialize)]
pub struct ApplicationConfig {
    /// 服务名称
    /// Service name
    pub server_name: String,
    /// 服务地址
    /// Service address
    pub server_url: String,
    /// 日志级别
    /// Log level
    pub log_level: String,
    /// 日志分割
    /// Log split
    pub log_split: String,
    /// 可执行文件目录
    /// Executable file directory
    pub exe_dir: PathBuf,
    /// 同步间隔
    /// Synchronization interval
    pub sync_interval: u64,
}
// TODO 这块默认配置应该还有优化的地方
impl Default for ApplicationConfig {
    fn default() -> Self {
        let exe_path = std::env::current_exe().expect("Failed to get current executable");
        let exe_dir = exe_path.parent().unwrap();
        let config_file = exe_dir.join("config.toml");
        let config_data = match std::fs::read_to_string(config_file) {
            Ok(data) => data,
            Err(_) => include_str!("../config.toml").to_owned(),
        };
        let result: ConfigFile = toml::from_str(&config_data).expect("load config file fail");
        let server_name = result
            .server_name
            .unwrap_or(env!("CARGO_PKG_NAME").to_owned());

        let port = result.port.unwrap_or(8000);
        let server_url = format!("0.0.0.0:{}", port);

        let log_level = result.log_level.unwrap_or("info".to_owned());
        let log_split = result.log_split.unwrap_or("day".to_owned());
        let sync_interval = result.sync_interval.unwrap_or(30);
        ApplicationConfig {
            server_name,
            server_url,
            log_level,
            log_split,
            exe_dir: exe_dir.to_path_buf(),
            sync_interval,
        }
    }
}
