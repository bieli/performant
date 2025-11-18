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
│ 0.05     │ 22808       │
│ 0.10     │ 3916        │
│ 0.15     │ 3992        │
│ 0.20     │ 4300        │
│ 0.25     │ 18208       │
│ 0.30     │ 51000       │
│ 0.35     │ 60264       │
│ 0.40     │ 66548       │
│ 0.45     │ 76980       │
│ 0.50     │ 79912       │
│ 0.55     │ 81724       │
│ 0.60     │ 85612       │
│ 0.65     │ 102192      │
│ 0.70     │ 133676      │
│ 0.75     │ 151800      │
│ 0.80     │ 167328      │
│ 0.85     │ 170240      │
│ 0.90     │ 199612      │
│ 0.95     │ 203240      │
│ 1.00     │ 225820      │
│ 1.05     │ 252852      │
│ 1.10     │ 256848      │
│ 1.15     │ 256848      │
│ 1.20     │ 260156      │
│ 1.25     │ 261692      │
│ 1.30     │ 274208      │
│ 1.35     │ 285204      │
│ 1.40     │ 291272      │
│ 1.45     │ 312252      │
│ 1.50     │ 316436      │
│ 1.55     │ 317736      │
│ 1.60     │ 333700      │
│ 1.65     │ 351652      │
│ 1.70     │ 369604      │
│ 1.75     │ 384116      │
│ 1.80     │ 318432      │
│ 1.85     │ 327932      │
│ 1.90     │ 345620      │
│ 1.95     │ 362780      │
│ 2.00     │ 379676      │
│ 2.05     │ 319524      │
│ 2.10     │ 320580      │
│ 2.15     │ 335628      │
│ 2.20     │ 352524      │
│ 2.25     │ 369948      │
│ 2.30     │ 378396      │
│ 2.35     │ 378396      │
│ 2.40     │ 378396      │
│ 2.45     │ 378396      │
│ 2.50     │ 378924      │
│ 2.55     │ 379116      │
│ 2.60     │ 379116      │
│ 2.65     │ 379116      │
│ 2.70     │ 379116      │
│ 2.75     │ 379116      │
│ 2.80     │ 384980      │
│ 2.85     │ 394668      │
│ 2.90     │ 394668      │
│ 2.95     │ 394668      │
│ 3.00     │ 394668      │
│ 3.05     │ 395092      │
│ 3.10     │ 402700      │
│ 3.15     │ 413080      │
│ 3.20     │ 413080      │
│ 3.25     │ 413688      │
│ 3.30     │ 413688      │
│ 3.35     │ 413952      │
│ 3.40     │ 421632      │
│ 3.46     │ 421632      │
│ 3.51     │ 422020      │
│ 3.56     │ 422020      │
│ 3.61     │ 422020      │
│ 3.66     │ 422020      │
│ 3.71     │ 422020      │
│ 3.76     │ 422020      │
│ 3.81     │ 422020      │
│ 3.86     │ 422020      │
│ 3.91     │ 422020      │
│ 3.96     │ 422020      │
│ 4.01     │ 422020      │
│ 4.06     │ 422020      │
│ 4.11     │ 422020      │
│ 4.16     │ 422020      │
│ 4.21     │ 422020      │
│ 4.26     │ 422020      │
│ 4.31     │ 422020      │
│ 4.36     │ 422020      │
│ 4.41     │ 422020      │
│ 4.46     │ 422020      │
│ 4.51     │ 420004      │
│ 4.56     │ 407248      │
│ 4.61     │ 407500      │
│ 4.66     │ 407500      │
│ 4.71     │ 391972      │
╰──────────┴─────────────╯

=== Memory Summary:
  Samples:   95
  Min:   12 KB
  Max:   422020 KB
  Avg:   313698.02 KB

=== Run Time Summary:
  Runs: 1
  Min:  4.783885630 sec
  Max:  4.783885630 sec
  Avg:  4.783885630 sec
```


## TODO list

- [ ] add CI
- [ ] publish package to crate.io
- [ ] warmup time as optional argument (means, we waiting N-seconds and collecting data from this point in time)
- [ ] add more unit tests
- [ ] add integration test with running on multiple Open Source packages/programs