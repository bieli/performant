use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct DurationStats {
    pub runs: usize,
    pub min: Duration,
    pub max: Duration,
    pub avg: Duration,
}

pub fn compute_duration_stats(durations: &[Duration]) -> Option<DurationStats> {
    if durations.is_empty() {
        return None;
    }

    let min = *durations.iter().min().unwrap();
    let max = *durations.iter().max().unwrap();
    let total: Duration = durations.iter().sum();
    let avg = total / (durations.len() as u32);

    Some(DurationStats {
        runs: durations.len(),
        min,
        max,
        avg,
    })
}

pub fn print_duration_stats(stats: &DurationStats) {
    println!("\n=== Run Time Summary:");
    println!("  Runs: {}", stats.runs);
    println!("  Min:  {:.9} sec", stats.min.as_secs_f64());
    println!("  Max:  {:.9} sec", stats.max.as_secs_f64());
    println!("  Avg:  {:.9} sec", stats.avg.as_secs_f64());
}
