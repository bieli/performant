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

## Real example - FreeCAD Open Source CAD profiling (at start moment)

```bash
$ cargo run --release -- -r 1 freecad
   Compiling performant v0.1.0 (...)
    Finished `release` profile [optimized] target(s) in 0.59s
     Running `target/release/performant -r 1 freecad`

>>> Run 1/1
FreeCAD 1.1.0, Libs: 1.1.0devR44225 (Git)
(C) 2001-2025 FreeCAD contributors
FreeCAD is free and open-source software licensed under the terms of LGPL2+ license.

Gtk-Message: 19:15:32.664: Failed to load module "canberra-gtk-module"
Gtk-Message: 19:15:32.664: Failed to load module "canberra-gtk-module"
Qt: Session management error: None of the authentication protocols specified are supported
connect failed: No such file or directory

╭────────────────────────╮
│ Memory Samples Summary │
├──────────┬─────────────┤
│ Time (s) │ Memory (KB) │
├──────────┼─────────────┤
│ 0.00     │ 12          │
│ 0.05     │ 22312       │
│ 0.10     │ 3776        │
│ 0.15     │ 3896        │
│ 0.20     │ 2740        │
│ 0.25     │ 8660        │
│ 0.30     │ 23712       │
│ 0.35     │ 56536       │
│ 0.40     │ 68648       │
│ 0.45     │ 77016       │
│ 0.50     │ 83088       │
│ 0.55     │ 132696      │
│ 0.60     │ 140708      │
│ 0.65     │ 151824      │
│ 0.70     │ 167864      │
│ 0.75     │ 175612      │
│ 0.80     │ 199672      │
│ 0.85     │ 203296      │
│ 0.90     │ 233324      │
│ 0.95     │ 254656      │
│ 1.00     │ 258460      │
│ 1.05     │ 261816      │
│ 1.10     │ 278060      │
│ 1.15     │ 286544      │
│ 1.20     │ 297740      │
│ 1.25     │ 313736      │
│ 1.30     │ 317776      │
│ 1.35     │ 323596      │
│ 1.40     │ 337852      │
│ 1.45     │ 354220      │
│ 1.50     │ 371908      │
│ 1.55     │ 393808      │
│ 1.60     │ 318364      │
│ 1.65     │ 332616      │
│ 1.70     │ 349248      │
│ 1.75     │ 365616      │
│ 1.80     │ 381192      │
│ 1.85     │ 319944      │
│ 1.90     │ 320472      │
│ 1.95     │ 333672      │
│ 2.00     │ 348720      │
│ 2.05     │ 363768      │
│ 2.10     │ 378288      │
│ 2.15     │ 378288      │
│ 2.20     │ 378288      │
│ 2.25     │ 378288      │
│ 2.30     │ 378288      │
│ 2.35     │ 378816      │
│ 2.40     │ 379212      │
│ 2.45     │ 379212      │
│ 2.50     │ 379212      │
│ 2.55     │ 379212      │
│ 2.60     │ 379212      │
│ 2.66     │ 382456      │
│ 2.71     │ 394764      │
│ 2.76     │ 394764      │
│ 2.81     │ 394764      │
│ 2.86     │ 394764      │
│ 2.91     │ 395204      │
│ 2.96     │ 399544      │
│ 3.01     │ 403496      │
│ 3.06     │ 412864      │
│ 3.11     │ 413260      │
│ 3.16     │ 413824      │
│ 3.21     │ 413824      │
│ 3.26     │ 414200      │
│ 3.31     │ 421652      │
│ 3.36     │ 421652      │
│ 3.41     │ 422536      │
│ 3.46     │ 422536      │
│ 3.51     │ 422536      │
│ 3.56     │ 422536      │
│ 3.61     │ 422536      │
│ 3.66     │ 422536      │
│ 3.71     │ 422536      │
│ 3.76     │ 422536      │
│ 3.81     │ 422536      │
│ 3.86     │ 422536      │
│ 3.91     │ 422536      │
│ 3.96     │ 422536      │
│ 4.01     │ 422536      │
│ 4.06     │ 422536      │
│ 4.11     │ 422536      │
│ 4.16     │ 422536      │
│ 4.21     │ 422536      │
│ 4.26     │ 422536      │
│ 4.31     │ 422536      │
│ 4.36     │ 422536      │
│ 4.41     │ 422536      │
│ 4.46     │ 422536      │
│ 4.51     │ 422536      │
│ 4.56     │ 409972      │
│ 4.62     │ 408100      │
│ 4.67     │ 408100      │
│ 4.72     │ 392572      │
│ 4.77     │ 392412      │
╰──────────┴─────────────╯

=== Memory Summary:
  Samples:   96
  Min:   12 KB
  Max:   422536 KB
  Avg:   325062.58 KB

=== Memory Usage Over Time (KB):
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⠤⠤⠒⠒⠒⠒⠊⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠒⠒⠢⣀⡀ 422536.0
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠔⠹⡀⠀⡠⠒⢇⠀⠀⠀⣀⠔⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠤⠔⠊⠀⠀⠀⠣⠊⠀⠀⠘⠤⠔⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠒⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⡠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⡠⠔⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⢀⢄⠀⠀⢀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠁⠀⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 12.0
0.0                                                                                    4.8


=== Run Time Summary:
  Runs: 1
  Min:  4.794981923 sec
  Max:  4.794981923 sec
  Avg:  4.794981923 sec
```


## TODO list

- [ ] add CI
- [ ] publish package to crate.io
- [ ] warmup time as optional argument (means, we waiting N-seconds and collecting data from this point in time)
- [ ] add more unit tests
- [ ] add integration test with running on multiple Open Source packages/programs
- [ ] add Caches profiling histogram
- [ ] CPU utilization profiling histogram
- [ ] IO utilization
- [ ] CPUs loads
