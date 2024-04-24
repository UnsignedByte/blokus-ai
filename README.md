# Instructions

## Run using

- `cargo run --profile=perf1`
- `cargo run --profile=perf2`
- ...

## Perf

```bash
PERF=1 bash -c 'cargo build --profile=perf$PERF && perf record target/perf$PERF/time'
perf report
```

## asm

- `cargo asm --lib --profile x86 blokus_ai::game::ver_x86::state::State::get_moves_for_piece`
