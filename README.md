# Advent of Code 2022

To run all days:

```sh
cargo run --release
```

Timings generated by:

The `cargo-criterion` crate is useful to get nice benchmarks.

```sh
cargo criterion
```

Though not required, this just doesn't have as nice output, and will deprecate plots soon:

```sh
cargo bench
```

## Timings

```
day01/get_input         time:   [45.651 µs 45.723 µs 45.814 µs]
day01/part1             time:   [550.68 ps 551.46 ps 552.34 ps]
day01/part2             time:   [2.3769 ns 2.3837 ns 2.3911 ns]


day02/get_input         time:   [45.677 µs 46.660 µs 47.879 µs]
day02/part1             time:   [7.4283 µs 7.4775 µs 7.5331 µs]
day02/part2             time:   [13.339 µs 13.472 µs 13.649 µs]

day03/get_input         time:   [48.540 µs 48.750 µs 49.069 µs]
day03/part1             time:   [133.71 ns 133.89 ns 134.08 ns]
day03/part2             time:   [116.03 ns 116.34 ns 116.69 ns]


day04/get_input         time:   [53.297 µs 53.618 µs 54.137 µs]
day04/part1             time:   [1.2248 µs 1.2271 µs 1.2296 µs]
day04/part2             time:   [2.1796 µs 2.1856 µs 2.1924 µs]

day05/get_input         time:   [33.789 µs 33.987 µs 34.206 µs]
day05/part1             time:   [5.4411 µs 5.5016 µs 5.5739 µs]
day05/part2             time:   [14.364 µs 14.828 µs 15.247 µs]

```

## Original Timings

```
day01/get_input         time:   [86.597 µs 86.773 µs 86.980 µs]
day01/part1             time:   [811.66 ns 813.55 ns 815.58 ns]
day01/part2             time:   [5.3042 µs 5.3207 µs 5.3401 µs]

day02/get_input         time:   [155.28 µs 157.07 µs 159.38 µs]
day02/part1             time:   [10.627 µs 10.668 µs 10.719 µs]
day02/part2             time:   [9.6086 µs 9.6317 µs 9.6583 µs]

day03/get_input         time:   [91.904 µs 93.008 µs 94.309 µs]
day03/part1             time:   [230.05 µs 234.30 µs 239.73 µs]
day03/part2             time:   [253.34 µs 256.95 µs 261.45 µs]

day04/get_input         time:   [53.297 µs 53.618 µs 54.137 µs]
day04/part1             time:   [1.2248 µs 1.2271 µs 1.2296 µs]
day04/part2             time:   [2.1796 µs 2.1856 µs 2.1924 µs]

day05/get_input         time:   [33.789 µs 33.987 µs 34.206 µs]
day05/part1             time:   [5.4411 µs 5.5016 µs 5.5739 µs]
day05/part2             time:   [14.364 µs 14.828 µs 15.247 µs]

```

## Failed experiments

To fill eventually
