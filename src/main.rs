use clap::Parser;
use libc::rusage;
use std::time::Duration;

use performant::measure::{print_memory_table, run_and_measure, MemoryStats};
use performant::stats::{compute_duration_stats, print_duration_stats};

#[derive(Parser, Debug)]
#[command(author, version, about = "performant: A CLI tool for profiling POSIX programs", long_about = None)]
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
    let final_samples: Vec<(f64, u64)> = Vec::new();
    let final_stats = MemoryStats {
        min: 0,
        max: 0,
        total: 0,
        samples: 0,
    };
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
        let (duration, usage, final_samples, final_stats) = run_and_measure(
            &args.program,
            &args.args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        );
        print_memory_table(&final_samples, &final_stats);
        durations.push(duration);
    }

    summarize_durations(&durations);
    println!(
        "\n===Mem Stats:\n  Max RSS (rusage): {} KB",
        usage.ru_maxrss
    );
}
