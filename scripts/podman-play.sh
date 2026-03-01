sudo podman run -d --replace --name track-pg -p 5432:5432 -v ~/testdb/trackpg:/var/lib/postgresql/data \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=task_tracker \
  docker.io/library/postgres

sudo podman run -d --replace --name track-nats -p 4222:4222 -p 8222:8222 -p 6222:6222 docker.io/library/nats

sudo podman run -d --replace --name track-redis -p 6379:6379 docker.io/library/redis

mkdir -p storage

sudo podman run -d --replace --name track-s3 -p 9000:9000 -p 9001:9001 -v ./storage:/data \
  quay.io/minio/minio server /data --console-address ":9001"

cleanup() {
    echo "Остановка..."
    sudo podman stop track-pg track-nats track-redis track-s3
    sudo podman rm track-pg track-nats track-redis track-s3
    exit 0
}

trap cleanup SIGINT

echo "Нажмите Ctrl+C для завершения..."

while :; do
    sleep 1
done
