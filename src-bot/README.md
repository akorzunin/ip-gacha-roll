- run dev

```
cd src-bot
cargo watch -x 'run'
```

run in docker ( from root dir )

```
docker compose up --build
```

- build for pi

```
cross build --release  --target armv7-unknown-linux-gnueabi
```

- build for arm

```
docker build -f src-bot/Dockerfile . --platform=linux/arm/v7
```
