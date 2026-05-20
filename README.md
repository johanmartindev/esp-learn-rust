## ESP32 Linker Workaround

This project uses a local linker workaround for ESP32 release builds.

When `esp-radio` is built with Wi-Fi, BLE, and coexistence enabled on `xtensa-esp32-none-elf`, the native archives from `esp-wifi-sys-esp32` can be linked in an order that leaves `libcoexist.a` unable to resolve `btdm_rf_bb_reg_init` from `libbtdm_app.a`.

The symptom looks like this:

```text
undefined reference to `btdm_rf_bb_reg_init`
```

To avoid patching vendored crates, [build.rs](./build.rs) appends `-lbtdm_app` at the end of the final link step for the `xtensa-esp32-none-elf` target. That gives the linker another chance to resolve the symbol after `libcoexist.a` has been processed.

If this workaround stops being necessary after an upstream crate update, the extra link argument in `build.rs` can be removed.

## Build Rust

- Make sure that local variables in `.env.local` are set correctly before running the above commands.

```sh
source .env.local && cargo build --release
source .env.local && cargo run --release
```

## Resources

- https://esp32.implrust.com/quick-start.html
