# fake-redis-rs
简单的实现redis协议的Server，为了熟悉rust语言

## 支持的命令
* `SET`
* `GET`
* `HSET`
* `HGET`
* `HDEL`
* `DEL`

## 测试
* 使用`redis-cli`
* 各语言的客户端库，如php的[`predis`](https://github.com/predis/predis)