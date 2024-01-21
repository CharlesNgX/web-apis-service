# web-apis-service
搭建一个稳定的web api服务


## Docker
```
# dev 
$ docker-compose --project-name web-apis-service up -d --build
$ docker-compose --project-name web-apis-service up -d

# test 
$ docker-compose -f docker-compose.yml -f docker-compose.test.yml up -d --build
$ docker-compose --project-name web-apis-service up -d

# prod
$ docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d --build
$ docker-compose --project-name web-apis-service up -d
```

## Diesel && Docker
```
$ docker-compose --project-name web-apis-service exec app  diesel setup
$ docker-compose --project-name web-apis-service exec app  diesel migration generate create_rustaceans
$ docker-compose --project-name web-apis-service exec app  diesel migration generate create_crates
$ docker-compose --project-name web-apis-service exec app  diesel migration run
```

## Schema.rs -> Model.rs
```
# 修改 rustaceans 和 crates 中的down.sql 和 up.sql 接着跑以下代码会生成 schema.rs
# 懒人福音，根据schema.rs 生成models.rs 类似于之前用Swift开发中 用json -> model 的工具类似
$ cd docker
$ docker-compose --project-name web-apis-service exec app diesel print-schema > ../src/schema.rs
$ docker-compose --project-name web-apis-service exec app diesel_ext --model > ../src/models.rs --add-table-name --rust_styled_model_fields --derive 'Insertable'
```