- run dev

```
cd src-bot
cargo watch -x 'run'
```

run in docker ( from root dir )

```
docker compose up --build
```

- NAT monitoring

`/natwatch` starts monitoring with `NAT_CHECK_INTERVAL_SECS` (300 seconds if unset).
`/natwatch 60` uses a custom interval; `/natstop` stops it. Status is sent when it changes.

- build for pi

```
cross build --release  --target armv7-unknown-linux-gnueabi
```

- build for arm

```
docker build -f src-bot/Dockerfile . --platform=linux/arm/v7
```
