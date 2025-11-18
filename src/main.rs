use clap::Parser;
use libc::rusage;
use std::time::Duration;

use performant::measure::run_and_measure;
use performant::stats::{compute_duration_stats, print_duration_stats};

/// performant: A CLI tool for profiling POSIX programs
#[derive(Parser, Debug)]
#[command(author, version, about = "performant - fetch OS performance metrics from running programs", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    runs: u32,

    #[arg(required = true)]
    program: String,

    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

fn summarize_durations(durations: &[Duration]) {
    if let Some(stats) = compute_duration_stats(durations) {
        print_duration_stats(&stats);
    } else {
        println!("No durations to summarize.");
    }
}

fn main() {
    let args = Args::parse();
    let mut durations = Vec::new();
    let usage = rusage {
        ru_utime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };

    for i in 0..args.runs {
        println!("\n>>> Run {}/{}", i + 1, args.runs);
        let (duration, usage) = run_and_measure(
            &args.program,
            &args.args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        );
        println!("Duration: {:?}", duration);
        durations.push(duration);
    }

    summarize_durations(&durations);
    println!("\n===Mem Stats:\n  Max RSS (rusage): {} KB", usage.ru_maxrss);
}
