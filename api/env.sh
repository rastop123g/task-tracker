sudo podman run -d --replace --name track-pg -p 5432:5432 -v ~/testdb/trackpg:/var/lib/postgresql/data \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=task_tracker \
  docker.io/library/postgres
