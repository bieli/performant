use std::time::Duration;
use std::time::Instant;

pub fn run_and_measure(_cmd: &str, _args: &[&str]) -> Duration {
    let start = Instant::now();
    let end = Instant::now();
    //TODO: implement measuring resource usage, add usage to return value
    end - start
}
