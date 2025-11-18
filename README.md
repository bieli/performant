# performant
Command line tool for fetch OS performance metrics from running programs in POSIX terminal

# Motivation
Sometimes, I need to know, what is better performance solution in on or another programming language, so I want to run like that:
`performant [-r 10] <program_binary> [args]`
to see more informations from runtime of `<program_binary>` (optional 10 runs `[-r 10]` one by one) like:
- RAM memory profiling histogram
- Caches profiling histogram
- CPU utilization profiling histogram
- IO utilization
- CPUs loads

This is not perfect performance test, but with this I will be happy to see, how program1 is more **performant** to the similar program2 and this is ok for now.

# How to run?

## Dev. run example

```bash
$ cargo run --release -- -r 3 ls

>>> Run 1/3
Duration: 74ns

>>> Run 2/3
Duration: 22ns

>>> Run 3/3
Duration: 16ns

=== Run Time Summary:
  Runs: 3
  Min:  0.000000016 sec
  Max:  0.000000074 sec
  Avg:  0.000000037 sec
```
