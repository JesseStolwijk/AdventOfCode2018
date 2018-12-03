# Advent of Code 2018 - Rust

Code written/tested with the following version of rust:
`rustc --version rustc 1.32.0-nightly (21f268495 2018-12-02)`

## Tooling

VSCode extensions:

- rust language support (rls)
- CodeLLDB for debugging
- crates, cargo dependency management
- code runner, easily test small code samples

#Benchmarks

I use the rust nightly `cargo bench` command to benchmark the code.
All messurements are in nanoseconds and may vary on other machines.

##Day 1
##Day 2 (With input parsing)

```
test day2::day2_tests::bench ... bench:     345,966 ns/iter (+/- 14,607)
```

##Day 3

```
test tests::bench ... bench:  63,024,707 ns/iter (+/- 1,210,708)
```
