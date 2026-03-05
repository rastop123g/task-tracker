# Task Tracker

## Зависимости
- postgresql
- redis
- nats
- s3 (minio)

## Первый запуск (api)
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
- Стандартно api запускается на http://localhost:8045
- Swagger документация на http://localhost:8045/api/docs

## Доп сведения по api
- Структуры протокола дополнительно реализуют ts_rs::TS для генерации typescript типов
- Весь протокол лежит в src/protocol/mod.rs
- Для генерации ts типов необходимо запускать тесты
```
cargo test
```
