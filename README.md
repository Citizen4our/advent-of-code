# Advent of Code

My solutions to [Advent of Code](https://adventofcode.com) puzzles.

## Edition

<details open><summary><h3><a href="https://adventofcode.com/2024">Advent of Code 2024</a></h3></summary>

> All benchmarks are run on a *MacBook Pro 14 inch* laptop with an *M3 Pro* CPU.

| D | Puzzle                                                    |              Code              | Part 1 Performance | Part 2 Performance |
|:-:|-----------------------------------------------------------|:------------------------------:|:------------------:|:------------------:|
| 1 | [Historian Hysteria](https://adventofcode.com/2024/day/1) | [day1.rs](aoc2024/src/day1.rs) |     60.833 µs      |      66.70 µs      |
| 2 | [Red-Nosed Reports](https://adventofcode.com/2024/day/2)  | [day2.rs](aoc2024/src/day2.rs) |     16.125 µs      |     184.458 µs     |
| 3 | [Mull It Over](https://adventofcode.com/2024/day/3)       | [day3.rs](aoc2024/src/day3.rs) |     585.62 µs      |     620.87 µs      |
| 5 | [Print Queue](https://adventofcode.com/2024/day/5)        | [day5.rs](aoc2024/src/day5.rs) |     150.68 µs      |     101.08 µs      |
| 5 | [Guard Gallivant](https://adventofcode.com/2024/day/6)    | [day6.rs](aoc2024/src/day6.rs) |     491.37 µs      |       4.03 s       |
</details>

## Instructions

### <a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/271355/rocket-ship-rocket.svg" width="14" height="14"></a> Get Answers and Run Performance Benchmarks

Thanks to [`cargo-aoc`](https://github.com/gobanos/cargo-aoc), answers and performance benchmarks for all days are obtainable by
running the main binary.

```bash
cargo run --release
```

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/269868/lab.svg" width="14" height="14"></a> Test the Correctness of Solutions

All days also include tests using sample inputs from the puzzle descriptions.

```bash
cargo test
```

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT)
or [Apache 2.0](LICENSE-APACHE) licenses.
