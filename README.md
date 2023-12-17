# Rust AOC 2023 solutions.

## Day 1-15 benchmarks.
PS C:\Projects\rust\aoc-2023> hyperfine .\target\release\aoc-2023.exe --warmup 10
Benchmark 1: .\target\release\aoc-2023.exe
  Time (mean ± σ):     144.8 ms ±   3.9 ms    [User: 125.0 ms, System: 6.1 ms]
  Range (min … max):   140.1 ms … 151.1 ms    20 runs

![Alt text](aoc-2023-all-days-profile.svg?raw=true "Flame graph of performance")
