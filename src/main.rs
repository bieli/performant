use clap::Parser;
use std::time::Duration;

use performant::measure::{
    compress_memory_samples, plot_memory_usage1, print_compressed_memory_summary,
    print_memory_summary, print_with_ascii_table_rs, run_and_measure,
};
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

    for i in 0..args.runs {
        println!("\n>>> Run {}/{}", i + 1, args.runs);
        let (duration, final_samples, final_stats) = run_and_measure(
            &args.program,
            &args.args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        );
        let compressed_memory_samples = compress_memory_samples(&final_samples);
        print_compressed_memory_summary(&compressed_memory_samples);
        print_memory_summary(&final_samples, &final_stats);
        plot_memory_usage1(&final_samples);
        durations.push(duration);
    }

    summarize_durations(&durations);
}
