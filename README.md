# performant
Command line tool for fetch OS performance metrics from running programs in POSIX terminal

# Motivation
Sometimes, I need to know, what is better performance solution in on or another programming language, so I want to run like that:
`performnat [-r 10] <program_binary> [args]`
to see more informations from runtime of `<program_binary>` (optional 10 runs `[-r 10]` one by one) like:
- RAM memory profiling histogram
- Caches profiling histogram
- CPU utilization profiling histogram
- IO utilization
- CPUs loads

This is not perfect performance test, but with this I will be happy to see, how program1 is more performant the similar program2 and this is ok for now.

#### TBD
