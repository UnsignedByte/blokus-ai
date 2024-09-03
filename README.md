# Blokus AI

A collection of AI Algorithms for the 4-player board game [Blokus](https://en.wikipedia.org/wiki/Blokus). Implements common algorithms including:
- Greedy
- Random
- MiniMax
- Monte Carlo
- Stochastic blends of other algorithms
All algorithms are implemented based on a heuristic figure. Heuristics implement the `Heuristic` Trait described [here](src/evaluate/algorithms/heuristics/heuristic.rs). Built in heuristics include
- Future fanout (number of possible future moves)
- Score
- Distance to a specified location
- Combinations of multiple heuristics

## Evaluation

Agents are evaluated using a standard ELO system. Each game of 4 players will be treated as 12 pairwise games, where the ELO of the player will increase or decrease based on whether they win or lose against every other player. In every tournament round, each player plays at least one game, where they choose 3 opponent agents within 250 ELO of themselves. Alternatively, Round Robin tournaments are also implemented but grow rapidly in time `O(n^4)` with the number of agents.

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
