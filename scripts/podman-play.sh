sudo podman run -d --replace --name track-pg -p 5432:5432 -v ~/testdb/trackpg:/var/lib/postgresql/data \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=task_tracker \
  docker.io/library/postgres

sudo podman run -d --replace --name track-nats -p 4222:4222 -p 8222:8222 -p 6222:6222 docker.io/library/nats

sudo podman run -d --replace --name track-redis -p 6379:6379 docker.io/library/redis

cleanup() {
    echo "Остановка..."
    sudo podman stop track-pg track-nats track-redis
    sudo podman rm track-pg track-nats track-redis
    exit 0
}

trap cleanup SIGINT

echo "Нажмите Ctrl+C для завершения..."

while :; do
    sleep 1
done
