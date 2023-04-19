use time::{macros::format_description, UtcOffset};
use tracing::{metadata::LevelFilter, subscriber};
use tracing_subscriber::{
    filter::Targets, fmt::time::OffsetTime, prelude::__tracing_subscriber_SubscriberExt, EnvFilter,
};

/// 初始化日志模块
/// Initialize log module
#[macro_export]
macro_rules! init_log {
    () => {
        let _guard = $crate::log::init();
    };
}

pub fn init() -> tracing_appender::non_blocking::WorkerGuard {
    // 设置时区
    // Set time zone
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    let fmt = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_time);

    // 如果是debug模式，日志输出到控制台，否则输出到文件
    // If it is debug mode, the log is output to the console, otherwise it is output to the file
    #[cfg(debug_assertions)]
    let (fmt, guard) = {
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
        let fmt = fmt
            .with_max_level(tracing::Level::DEBUG)
            .with_ansi(true)
            .with_writer(non_blocking)
            .pretty();
        (fmt, guard)
    };

    #[cfg(not(debug_assertions))]
    let (fmt, guard) = {
        use super::CONFIG;

        let log_level = match &CONFIG.log_level[..] {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        };

        let exe_path = std::env::current_exe().expect("Failed to get current executable");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");
        let log_dir = exe_dir.join("logs");

        if !log_dir.exists() {
            std::fs::create_dir(&log_dir).expect("Failed to create log directory");
        }
        let log_file_name = format!("{}.log", &CONFIG.server_name);
        let file_appender = match &CONFIG.log_split[..] {
            "hour" => tracing_appender::rolling::hourly(log_dir, &log_file_name),
            "minute" => tracing_appender::rolling::minutely(log_dir, &log_file_name),
            _ => tracing_appender::rolling::daily(log_dir, &log_file_name),
        };

        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let fmt = fmt
            .with_max_level(log_level)
            .with_ansi(false)
            .with_writer(non_blocking);
        (fmt, guard)
    };

    let targets = Targets::new()
        .with_target("h2", LevelFilter::OFF)
        .with_default(LevelFilter::DEBUG);

    let fmt = fmt.finish().with(targets);

    subscriber::set_global_default(fmt).unwrap();

    guard
}
