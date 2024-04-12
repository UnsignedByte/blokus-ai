# Instructions

## Run using

- `cargo run --profile=perf`
- `cargo run --profile=x86`

## Perf

### Normal ver

- `cargo build --profile=perf && perf record target/perf/blokus-ai`
- `perf report`

### AVX ver

- `cargo build --profile=x86 && perf record target/x86/blokus-ai`
- `perf report`

## asm

- `cargo asm --lib --profile x86 blokus_ai::game::ver_x86::state::State::get_moves_for_piece`
