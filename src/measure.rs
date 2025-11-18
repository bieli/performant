use std::fs;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use ascii_table_rs::{AsciiTable, CellValue};
use textplots::{Chart, Plot, Shape};
use textplots::utils::histogram;

#[derive(Debug)]
pub struct MemoryStats {
    pub min: u64,
    pub max: u64,
    pub total: u64,
    pub samples: u64,
}

pub fn monitor_memory(
    pid: u32,
    stats: Arc<Mutex<MemoryStats>>,
    samples: Arc<Mutex<Vec<(f64, u64)>>>,
) {
    let path = format!("/proc/{}/status", pid);
    let start = Instant::now();
    while fs::metadata(&path).is_ok() {
        if let Ok(contents) = fs::read_to_string(&path) {
            for line in contents.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Ok(mem_kb) = parts[1].parse::<u64>() {
                        let elapsed = start.elapsed().as_secs_f64();
                        samples.lock().unwrap().push((elapsed, mem_kb));
                        let mut stats = stats.lock().unwrap();
                        stats.min = stats.min.min(mem_kb);
                        stats.max = stats.max.max(mem_kb);
                        stats.total += mem_kb;
                        stats.samples += 1;
                    }
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

pub fn print_memory_summary(samples: &[(f64, u64)], stats: &MemoryStats) {
    println!("\n=== Memory Summary:");
    println!("  Samples:   {}", stats.samples);
    println!("  Min:   {} KB", stats.min);
    println!("  Max:   {} KB", stats.max);
    println!(
        "  Avg:   {:.2} KB",
        stats.total as f64 / stats.samples as f64
    );
}

pub fn print_with_ascii_table_rs(samples: &[(f64, u64)], stats: &MemoryStats) {
    let mut table = AsciiTable::new("Memory Samples Summary");
    table.set_headers(vec!["Time (s)", "Memory (KB)"]);
    table.set_decimal_places(2);

    for (t, mem) in samples {
        table.add_row(vec![
            CellValue::Float(*t),
            CellValue::Int(*mem as i64),
        ]);
    }

    table.render();
}

pub fn plot_memory_usage1(samples: &[(f64, u64)]) {
    if samples.is_empty() {
        println!("No memory samples to plot.");
        return;
    }

    let points: Vec<(f32, f32)> = samples
        .iter()
        .map(|(t, mem)| (*t as f32, *mem as f32))
        .collect();

    println!("\n=== Memory Usage Over Time (KB):");
    Chart::new(180, 40, points[0].0, points.last().unwrap().0)
        .lineplot(&Shape::Lines(&points))
        .display();
}

pub fn plot_memory_usage2(samples: &[(f64, u64)]) {
    if samples.is_empty() {
        println!("No memory samples to plot.");
        return;
    }

    let points: Vec<(f32, f32)> = samples
        .iter()
        .map(|(t, mem)| (*t as f32, *mem as f32))
        .collect();

    let min_y = points.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
    let max_y = points.iter().map(|(_, y)| *y).fold(f32::NEG_INFINITY, f32::max);

    println!("\n=== Memory Usage Over Time (KB)");
    println!("Y-axis range: {:.0} KB → {:.0} KB", min_y, max_y);

    Chart::new(120, 20, points[0].0, points.last().unwrap().0)
        .lineplot(&Shape::Lines(&points))
        .display();

    println!("X-axis: {:.1} sec → {:.1} sec", points[0].0, points.last().unwrap().0);
}


pub fn print_memory_histogram(samples: &[(f64, u64)], bin_size: u64) {
    if samples.is_empty() {
        println!("No memory samples to plot.");
        return;
    }

    let mem_values: Vec<u64> = samples.iter().map(|(_, mem)| *mem).collect();
    let min = *mem_values.iter().min().unwrap();
    let max = *mem_values.iter().max().unwrap();

    let mut bins = std::collections::BTreeMap::new();
    let mut bin_start = min - (min % bin_size);
    while bin_start <= max {
        bins.insert(bin_start, 0);
        bin_start += bin_size;
    }

    for &mem in &mem_values {
        let bin = mem - (mem % bin_size);
        if let Some(count) = bins.get_mut(&bin) {
            *count += 1;
        }
    }

    println!("\n=== Memory Usage Histogram (KB):");
    println!("  Range: {} KB → {} KB\n", min, max);

    for (&bin, &count) in bins.iter().rev() {
        let bar = "▇".repeat(count);
        println!("{:>6} ┤{}", bin, bar);
    }
}

pub fn run_and_measure(cmd: &str, args: &[&str]) -> (Duration, Vec<(f64, u64)>, MemoryStats) {
    let start = Instant::now();
    let mut child = Command::new(cmd)
        .args(args)
        .spawn()
        .expect("Failed to start process");

    let pid = child.id();
    let stats = Arc::new(Mutex::new(MemoryStats {
        min: u64::MAX,
        max: 0,
        total: 0,
        samples: 0,
    }));
    let samples = Arc::new(Mutex::new(Vec::new()));

    let stats_clone = Arc::clone(&stats);
    let samples_clone = Arc::clone(&samples);
    let monitor_thread = thread::spawn(move || {
        monitor_memory(pid, stats_clone, samples_clone);
    });

    let _ = child.wait().expect("Failed to wait on child");
    let end = Instant::now();
    monitor_thread.join().unwrap();

    let final_stats: MemoryStats = Arc::try_unwrap(stats).unwrap().into_inner().unwrap();
    let final_samples: Vec<_> = Arc::try_unwrap(samples).unwrap().into_inner().unwrap();

    (end - start, final_samples, final_stats)
}
