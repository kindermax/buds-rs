# Buds-rs is a tool to communicate with bluetooth headphones (earbuds)


## Usage

Show battery level

```
buds-rs --mac 6C:0D:E1:9C:0E:E1 -c 19   # -c is an optional RFCOMM device channel
```
## Install

First you need to install some system packages

```
sudo apt install libdbus-1-dev pkg-config
```

Then run

`lets install <path>`, for example `lets install ~/bin` (if ~/bin in $PATH)

This will install `buds-rs` binary into your system.

## Build

`lets build`

## TODO

- [x] Show battery level
- [ ] Adjust volume