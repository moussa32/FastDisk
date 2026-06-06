use std::time::{Duration, Instant};

use super::ProgressThrottle;

#[test]
fn throttle_allows_first_event() {
    let mut throttle = ProgressThrottle::new(Duration::from_millis(250));
    assert!(throttle.should_emit(Instant::now()));
}

#[test]
fn throttle_blocks_events_inside_interval() {
    let mut throttle = ProgressThrottle::new(Duration::from_millis(250));
    let now = Instant::now();
    assert!(throttle.should_emit(now));
    assert!(!throttle.should_emit(now + Duration::from_millis(100)));
}

#[test]
fn throttle_allows_events_after_interval() {
    let mut throttle = ProgressThrottle::new(Duration::from_millis(250));
    let now = Instant::now();
    assert!(throttle.should_emit(now));
    assert!(throttle.should_emit(now + Duration::from_millis(250)));
}
