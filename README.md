# node_service

## 运行方法

```shell
# 启动 postgres 数据库，并建表
# 由于 sqlx 宏会在编译前连接数据库进行数据类型检查，所以提前建表是必须的，如果没有这一步，你甚至不能编译
docker compose -f ./script/compose/postgres/compose.yaml up -d
# 运行
cargo run
```

## 技术栈

- axum
- sqlx
- postgres
