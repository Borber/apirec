中文 | [English](./README_EN.md)

# Apirec

简单的 Api 记录工具

## 理念

-   快速
-   轻量
-   单文件
-   持久化

## 部署

下载可执行文件直接运行即可

## 接口

### 添加 App

接口地址: `127.0.0.1:8000/api`

请求方式: `POST`

请求参数:

```json
{
    "app": "test1"
}
```

样例返回:

```json
{
    "code": 0,
    "msg": "success",
    "data": "Add app success"
}
```

### 向 App 添加 Api

接口地址: `127.0.0.1:8000/api/test1`

请求方式: `POST`

请求参数:

```json
{
    "api": "ttt1"
}
```

样例返回:

```json
{
    "code": 0,
    "msg": "success",
    "data": "Success"
}
```

### 添加 Api 调用记录

接口地址: `127.0.0.1:8000/api/test1/ttt1`

请求方式: `POST`

请求参数: 无

样例返回:

```json
{
    "code": 0,
    "msg": "success",
    "data": 2498788
}
```

### 获取 Api 调用记录

接口地址: `127.0.0.1:8000/api/test1/ttt1`

请求方式: `GET`

请求参数: 无

样例返回:

```json
{
    "code": 0,
    "msg": "success",
    "data": 2498788
}
```

### 获取 App 下所有 Api 调用记录

接口地址: `127.0.0.1:8000/api/test1`

请求方式: `GET`

请求参数:

```json
{
    "limit": 2,
    "sort": true,
    "apis": ["ttt1", "ttt2"]
}
```

-   limit: 限制返回条数
-   sort: 是否排序
-   apis: 指定需要返回的 api

样例返回:

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "total": 2498788,
        "apis": [
            {
                "api": "ttt1",
                "count": 2498788
            },
            {
                "api": "ttt2",
                "count": 0
            }
        ]
    }
}
```

## 设置

```toml
#名称
server_name = "apirec"
#服务端口
port = 8000
#日志级别
log_level = "info"
#日志分割 day, hour, minute
log_split = "day"
#同步间隔(秒)
sync_interval = 30

```

## 基准测试

目前没有找到合适的测试方法, 目前在我的笔记本上测试结果如下

-   CPU 12th Gen Intel(R) Core(TM) i7-1255U
-   内存 16G 4267MHz LPDDR4x

```bash
.\rsb.exe -l -m POST -d 10  http://127.0.0.1:8000/api/test1/ttt1
Post "http://127.0.0.1:8000/api/test1/ttt1" with for 10s using 50 connections
▪▪▪▪▪ [00:00:10] [####################] 10s/10s (100%)
Statistics         Avg          Stdev          Max
  Reqs/sec       86257.56      1482.19       88056.00
  Latency        575.64µs      148.94µs       4.09ms
  Latency Distribution
     50%     460.55µs
     75%     510.74µs
     90%     543.59µs
     99%     570.45µs
  HTTP codes:
    1XX - 0, 2XX - 859995, 3XX - 0, 4XX - 0, 5XX - 0
    others - 0
  Throughput:   86859.69/s
```

平均每秒 **8.6** 万次请求

## 许可证

MIT License
