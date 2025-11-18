use libc::{getrusage, rusage, RUSAGE_CHILDREN};
use std::process::Command;
use std::time::Duration;
use std::time::Instant;

pub fn run_and_measure(cmd: &str, args: &[&str]) -> (Duration, rusage) {
    let start = Instant::now();
    let mut child = Command::new(cmd)
        .args(args)
        .spawn()
        .expect("Failed to start process");

    let _ = child.wait().expect("Failed to wait on child");
    let end = Instant::now();

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

    (end - start, usage)
}
