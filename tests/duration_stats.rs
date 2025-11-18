use performant::stats::compute_duration_stats;
use std::time::Duration;

#[test]
fn test_empty_input_returns_none() {
    let result = compute_duration_stats(&[]);
    assert_eq!(result, None);
}

#[test]
fn test_single_duration() {
    let d = Duration::from_millis(100);
    let result = compute_duration_stats(&[d]).unwrap();
    assert_eq!(result.runs, 1);
    assert_eq!(result.min, d);
    assert_eq!(result.max, d);
    assert_eq!(result.avg, d);
}

#[test]
fn test_multiple_durations() {
    let d1 = Duration::from_millis(100);
    let d2 = Duration::from_millis(200);
    let d3 = Duration::from_millis(300);
    let result = compute_duration_stats(&[d1, d2, d3]).unwrap();

    assert_eq!(result.runs, 3);
    assert_eq!(result.min, d1);
    assert_eq!(result.max, d3);
    assert_eq!(result.avg, Duration::from_millis(200));
}
