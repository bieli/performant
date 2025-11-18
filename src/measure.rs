use libc::{getrusage, rusage, RUSAGE_CHILDREN};
use std::fs;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

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

pub fn print_memory_table(samples: &[(f64, u64)], stats: &MemoryStats) {
    println!("\n┌────────────┬───────────────┐");
    println!("│ Time (s)   │ Memory (KB)   │");
    println!("├────────────┼───────────────┤");
    for (t, mem) in samples {
        println!("│ {:>10.2} │ {:>13} │", t, mem);
    }
    println!("└────────────┴───────────────┘");
    println!("\n=== Memory Summary:");
    println!("  Samples:   {}", stats.samples);
    println!("  Min:   {} KB", stats.min);
    println!("  Max:   {} KB", stats.max);
    println!(
        "  Avg:   {:.2} KB",
        stats.total as f64 / stats.samples as f64
    );
}

pub fn run_and_measure(
    cmd: &str,
    args: &[&str],
) -> (Duration, rusage, Vec<(f64, u64)>, MemoryStats) {
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

    let usage = {
        let mut usage = rusage {
            ru_utime: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            ru_stime: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            ru_maxrss: 0,
            ..unsafe { std::mem::zeroed() }
        };
        unsafe { getrusage(RUSAGE_CHILDREN, &mut usage) };
        usage
    };

    let final_stats: MemoryStats = Arc::try_unwrap(stats).unwrap().into_inner().unwrap();
    let final_samples: Vec<_> = Arc::try_unwrap(samples).unwrap().into_inner().unwrap();

    (end - start, usage, final_samples, final_stats)
}
