use clap::Parser;
use std::time::Duration;
use std::time::Instant;

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

fn run_and_measure(_cmd: &str, _args: &[&str]) -> Duration {
    let start = Instant::now();
    let end = Instant::now();
    //TODO: implement measuring resource usage, add usage to return value
    end - start
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

    for i in 0..args.runs {
        println!("\n>>> Run {}/{}", i + 1, args.runs);
        let duration = run_and_measure(
            &args.program,
            &args.args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        );
        println!("Duration: {:?}", duration);
        durations.push(duration);
    }

    summarize_durations(&durations);
}
