use speakr_core::transcription::performance::PerformanceMonitor;
use std::time::Duration;

#[test]
fn monitor_records_latency() {
    let monitor = PerformanceMonitor::new();

    monitor.run("sleep", || {
        std::thread::sleep(Duration::from_millis(30));
    });

    let metrics = monitor.metrics();
    assert_eq!(metrics.len(), 1);
    assert!(metrics[0].duration >= Duration::from_millis(30));
}
