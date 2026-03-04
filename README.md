# Task Tracker

## Зависимости
- postgresql
- redis
- nats
- s3 (minio)

## Первый запуск
- Запускаем все контейнеры (дефолтные порты/настройки смотреть в ./api/.env и ./api/src/config.rs)
- Создаем бакет (название в ./api/src/config.rs) можно с использованием скрипта ./scripts/create_local_bucket.sh
- Устанавливаем sqlx-cli
```
cargo install sqlx-cli
```
- Делаем первую миграцию (обязательно, иначе api даже не скомпилируется)
```
cargo sqlx migrate run
```
- Запускаем api
```
cargo run -- serve
```
