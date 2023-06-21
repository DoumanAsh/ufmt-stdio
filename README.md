# ufmt-stdio

[![Crates.io](https://img.shields.io/crates/v/ufmt-stdio.svg)](https://crates.io/crates/ufmt-stdio)
[![Documentation](https://docs.rs/ufmt-stdio/badge.svg)](https://docs.rs/crate/ufmt-stdio/)
[![Build](https://github.com/DoumanAsh/ufmt-stdio/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/ufmt-stdio/actions?query=workflow%3ARust)

Minimal printing facilities for [ufmt](https://github.com/japaric/ufmt)

## Supported platforms:

- `wasm` via `wasm-bindings`;
- `mos` microprocessors via linking `putchar`;
- `riscv32` via `esp-*` features (see details in `Features` section) or being no-op otherwise;
- All other platforms are built upon standard C library `write` function.

## Features

- `esp-uart` - Enables UART writer on `riscv32` targets. Mutually exclusive with `esp-jtag`. Requires user to provide symbols:
  - `ESP_UART_ADDR` (e.g. on ESP32-C3 it is `#[no_mangle] static ESP_UART_ADDR: usize = 0x40000068`).
- `esp-jtag` - Enables JTAG writer on `riscv32` targets. Mutually exclusive with `esp-jtag`. Requires user to provide symbols:
  - `SERIAL_JTAG_FIFO_REG` (e.g. on ESP32-C3 it is `#[no_mangle] static SERIAL_JTAG_FIFO_REG: usize = 0x60043000`)
  - `SERIAL_JTAG_CONF_REG` (e.g. on ESP32-C3 it is `#[no_mangle] static SERIAL_JTAG_CONF_REG: usize = 0x60043004`).
