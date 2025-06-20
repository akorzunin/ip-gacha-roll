# ip-gacha-roll

Program that resets IP address

# Why?

well, imagine u have a home server and wanna some of ur services be available from internet

and also u don't wanna have static ip address

and also ur provider sometimes gives you ip that is not hidden behind a NAT

and sometimes its just works

so the only solution i came up to is ROLL ip addresses until provider gives me one that works

# How?

Make REST requests to router thst basically do

```
interface PPPoE0 down
interface PPPoE0 up
```

and then check if ip is ok by ICMP request to whatewer ip comes from ifconfig.me

```
ping $(curl ifconfig.me) -W 10 -c 1
```

if exit code is 0, then ip is ok, otherwise roll ip address again

## Screenshot

![Image](https://github.com/user-attachments/assets/0dbb1691-471f-4d64-ab14-d8cc3c8114c5)

# Run dev

```
pnpm install
pnpm tauri dev
```

# Build

```
pnpm tauri build
```
