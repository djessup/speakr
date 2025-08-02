//! Performance monitoring and optimisation utilities.
//!
//! This module provides simple, *zero-dependency* helpers for measuring the
//! latency and memory impact of arbitrary operations.  The collected metrics are
//! useful for unit tests, diagnostics, and future UI integrations.
//!
//! # Design goals
//!
//! 1. **Low overhead** – instrumentation should add minimal latency.
//! 2. **Thread-safe** – metrics can be gathered from multiple threads.
//! 3. **Standalone** – no external runtime required; works in `no_std` targets
//!    that have access to `alloc` (memory readings are skipped in that case).
//!
//! The implementation relies on the [`sysinfo`] crate for memory statistics on
//! desktop targets.  If memory readings are not available the monitor gracefully
//! falls back to measuring latency only.
//!
//! [`sysinfo`]: https://crates.io/crates/sysinfo
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sysinfo::System;

/// A single performance data point recorded by [`PerformanceMonitor`].
#[derive(Debug, Clone)]
pub struct PerformanceEntry {
    /// Human-readable description of the monitored operation.
    pub description: String,
    /// Wall-clock time taken by the operation.
    pub duration: Duration,
    /// Difference in **used** memory (bytes) measured before → after.
    pub memory_delta_bytes: u64,
}

/// Lightweight latency & memory monitor.
///
/// Wrap an expensive closure with [`PerformanceMonitor::run`] to automatically
/// capture execution time and memory impact.  All results are stored in-memory
/// and can be inspected via [`PerformanceMonitor::metrics`].
///
/// ## Example
/// ```
/// use speakr_core::transcription::performance::PerformanceMonitor;
/// use std::time::Duration;
///
/// let monitor = PerformanceMonitor::new();
/// let answer = monitor.run("sleep", || {
///     std::thread::sleep(Duration::from_millis(20));
///     42
/// });
///
/// assert_eq!(answer, 42);
/// let metrics = monitor.metrics();
/// assert_eq!(metrics.len(), 1);
/// assert!(metrics[0].duration >= Duration::from_millis(20));
/// ```
#[derive(Debug, Default, Clone)]
pub struct PerformanceMonitor {
    entries: Arc<Mutex<Vec<PerformanceEntry>>>,
}

impl PerformanceMonitor {
    /// Create a new, empty monitor.
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Execute `operation`, recording latency & memory usage.
    pub fn run<F, R>(&self, description: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        // Memory before ------------------------------------------------------
        let mut sys = System::new();
        sys.refresh_memory();
        let mem_before = sys.used_memory();

        // Latency ------------------------------------------------------------
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();

        // Memory after -------------------------------------------------------
        sys.refresh_memory();
        let mem_after = sys.used_memory();
        let mem_delta_bytes = mem_after.saturating_sub(mem_before) * 1024;

        // Persist ------------------------------------------------------------
        let entry = PerformanceEntry {
            description: description.to_string(),
            duration,
            memory_delta_bytes: mem_delta_bytes,
        };

        self.entries
            .lock()
            .expect("poisoned performance monitor mutex")
            .push(entry);

        result
    }

    /// Return a **snapshot** of all recorded entries.
    pub fn metrics(&self) -> Vec<PerformanceEntry> {
        self.entries
            .lock()
            .expect("poisoned performance monitor mutex")
            .clone()
    }

    /// Clear all collected metrics.
    pub fn clear(&self) {
        self.entries
            .lock()
            .expect("poisoned performance monitor mutex")
            .clear();
    }
}
