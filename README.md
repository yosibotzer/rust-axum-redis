## RUST-AXUM-REDIS Service

Example of rest API, reading and writing from Redis, written in Rust and using the Axum framework


### Build service docker
```
docker build --progress=plain -t rust-axum-redis .
```

### Start Postgres docker
```
docker run -d --name redis-stack -p 6379:6379 -p 8001:8001 redis/redis-stack:latest
```

### Start service docker
```
docker run --rm -d --network host --name my-rust-axum-redis rust-axum-redis
```



