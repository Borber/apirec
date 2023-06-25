[中文](./README.md) | English

# Apirec

A simple api record tool

## Philosophy

-   Fast
-   Lightweight
-   Single file
-   Persistence

## Deploy

Just download the executable file and run it directly

## Interface

### Adding App

address: `127.0.0.1:8000/api`

method: `POST`

params:

```json
{
    "app": "test1"
}
```

Sample returns:

```json
{
    "code": 0,
    "msg": "success",
    "data": "Add app success"
}
```

### Adding an Api to an App

address: `127.0.0.1:8000/api/test1`

method: `POST`

params:

```json
{
    "api": "ttt1"
}
```

Sample returns:

```json
{
    "code": 0,
    "msg": "success",
    "data": "Success"
}
```

### Adding Api Call Records

address: `127.0.0.1:8000/api/test1/ttt1`

method: `POST`

params: None

Sample returns:

```json
{
    "code": 0,
    "msg": "success",
    "data": 2498788
}
```

### Get Api call records

address: `127.0.0.1:8000/api/test1/ttt1`

method: `GET`

params: None

Sample returns:

```json
{
    "code": 0,
    "msg": "success",
    "data": 2498788
}
```

### Get all Api call records under App

address: `127.0.0.1:8000/api/test1`

method: `GET`

params:

```json
{
    "limit": 2,
    "sort": true,
    "apis": ["ttt1", "ttt2"]
}
```

-   limit: Limit the number of return items
-   sort: Sort or not
-   apis: Specify the api to be returned

Sample returns:

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

## Configuration

```toml
server_name = "apirec"
port = 8000
log_level = "info"
# day, hour, minute
log_split = "day"
# Sync interval (sec)
sync_interval = 30

```

## Benchmarking

No suitable test method has been found, and the results on my laptop so far are as follows

-   CPU 12th Gen Intel(R) Core(TM) i7-1255U
-   Memory 16G 4267MHz LPDDR4x

```bash
 .\rsb.exe -l -m POST -d 10  http://127.0.0.1:8000/api/test1/ttt1
Post "http://127.0.0.1:8000/api/test1/ttt1" with for 10s using 50 connections
▪▪▪▪▪ [00:00:10] [####################] 10s/10s (100%)
Statistics         Avg          Stdev          Max
  Reqs/sec       85716.89       663.19       86343.00
  Latency        578.39µs      152.24µs       4.40ms
  Latency Distribution
     50%     462.04µs
     75%     512.68µs
     90%     545.87µs
     99%     572.98µs
  HTTP codes:
    1XX - 0, 2XX - 855982, 3XX - 0, 4XX - 0, 5XX - 0
    others - 0
  Throughput:   86447.16/s
```

Average of **8.6** million requests per second

## License

MIT License
