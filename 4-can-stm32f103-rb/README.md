## For chip STM32F103RB & cortex-M3
## Install tools
Install flip-link, cargo-generate and probe-run
```
cargo install flip-link
cargo install cargo-generate
cargo install probe-run
```

## Update Config
Generate Cargo.toml
```
cargo generate \
    --git https://github.com/knurling-rs/app-template \
    --branch main \
    --name my-app
```

Add to .cargo/config.toml:
```
[build]
target = "thumbv7m-none-eabi"    # Cortex-M3

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
[target.thumbv7m-none-eabi]
runner = "probe-run --chip STM32F103RB"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

Add to Cargo.toml template:
```
[dependencies]
stm32f1xx-hal = { version = "0.9.0", features = ["rt", "stm32f103", "medium"] }
embedded-hal = "0.2.3"
nb = "0.1.2"
panic-halt = "0.2.0"
bxcan = "0.6.0"
```



