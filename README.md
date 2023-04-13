中文 | English

## 目的

收集 Api 调用次数, 以便于分析 Api 调用情况, 以及优化 Api 调用

## 理念

-   轻量化
-   独立任务线程操作数据库
-   单文件部署

## 部署

## 设置

软件默认配置

```toml
#名称 将显示在日志中
server_name = "apirec"
#server 监听地址
server_url = "0.0.0.0:3006"
#日志目录
log_dir = "logs"
#数据库文件名
log_temp_size = "100MB"
#日志级别
log_level = "info"

```

## 感谢

## 许可证

MIT License
