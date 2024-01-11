# web-apis-service
搭建一个稳定的web api服务


## Docker
```
$ docker-compose up -d
$ docker-compose up -d --build
$ docker-compose logs postgres
```

## Diesel && Docker
```
$ docker-compose up -d
$ docker-compose exec app  diesel setup
$ docker-compose exec app  diesel migration generate create_rustaceans
$ docker-compose exec app  diesel migration generate create_crates
```

postgres容器可能报一些错误：
`postgres FATAL:  no pg_hba.conf entry for host "172.19.0.4"`
使用一下命令解决：
```
$ docker exec -it web-apis-service-postgres-1 bash
$ find /etc/postgresql -name "pg_hba.conf"
# 使用docker客户端，找到文件路径后，添加端口
$ host    all             all             172.19.0.0/16             md5
# 接着重启postgres容器
$ docker restart web-apis-service-postgres-1
$ docker-compose exec app  diesel setup
$ ......
```

## Schema.rs -> Model.rs
```
# 修改 rustaceans 和 crates 中的down.sql 和 up.sql 接着跑以下代码会生成 schema.rs
$ docker-compose exec app  diesel migration run
# 懒人福音，根据schema.rs 生成models.rs 类似于之前用Swift开发中 用json -> model 的工具类似
$ docker-compose exec app diesel print-schema > src/schema.rs
$ docker-compose exec app diesel_ext --model > src/models.rs
```