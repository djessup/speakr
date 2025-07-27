# Design Document

## Overview

The latency performance system ensures Speakr meets its sub-3-second response time target through
comprehensive measurement, monitoring, and validation infrastructure. This design focuses on
creating automated testing and telemetry systems that validate performance requirements across the
entire dictation pipeline.

## Architecture

### Latency Measurement Pipeline

```text
Hotkey Activation → Audio Capture → Transcription → Text Injection
       ↓               ↓              ↓              ↓
   Timestamp T1    Timestamp T2   Timestamp T3   Timestamp T4
       ↓               ↓              ↓              ↓
                    Telemetry Collection System
                              ↓
                    Performance Analysis & Reporting
```

### Components Overview

1. **Telemetry Collection**: Embedded measurement points throughout the dictation pipeline
2. **Performance Testing Framework**: Automated tests for CI/CD validation
3. **Regression Detection**: Continuous monitoring and alerting for performance degradation
4. **Reporting Dashboard**: Real-time and historical performance analytics

## Components and Interfaces

### Telemetry Collection System

**Location**: `speakr-core/src/telemetry/`

**Responsibilities**:

- Capture high-precision timestamps at key pipeline stages
- Calculate end-to-end and component-level latencies
- Store measurements for analysis and reporting
- Provide APIs for performance test integration

**Key Interfaces**:

```rust
pub struct LatencyTracker {
    start_time: Instant,
    checkpoints: HashMap<String, Instant>,
}

impl LatencyTracker {
    pub fn start() -> Self;
    pub fn checkpoint(&mut self, stage: &str);
    pub fn finish(self) -> LatencyMeasurement;
}

pub struct LatencyMeasurement {
    pub total_duration: Duration,
    pub stage_durations: HashMap<String, Duration>,
    pub timestamp: SystemTime,
}
```

### Performance Testing Framework

**Location**: `speakr-tauri/tests/performance/`

**Responsibilities**:

- Execute automated latency tests in CI environment
- Simulate various audio clip lengths and system conditions
- Validate P95 latency targets across different scenarios
- Generate performance reports for build validation

**Test Categories**:

1. **Baseline Tests**: Standard 5-second clips with small model
2. **Model Comparison**: Latency across small/medium/large models
3. **Audio Length Variation**: 1s, 3s, 5s, 10s clip performance
4. **System Load Tests**: Performance under CPU/memory pressure

### Regression Detection System

**Location**: `speakr-tauri/src/monitoring/`

**Responsibilities**:

- Continuous monitoring of latency metrics
- Statistical analysis for trend detection
- Automated alerting for performance degradation
- Integration with CI/CD pipeline for build gating

## Data Models

### LatencyMeasurement Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMeasurement {
    pub session_id: Uuid,
    pub timestamp: SystemTime,
    pub total_duration_ms: u64,
    pub audio_capture_ms: u64,
    pub transcription_ms: u64,
    pub text_injection_ms: u64,
    pub model_size: String,
    pub audio_length_ms: u64,
    pub system_load: SystemLoadMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLoadMetrics {
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: u64,
    pub background_processes: u32,
}
```

### Performance Report Structure

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub test_run_id: Uuid,
    pub timestamp: SystemTime,
    pub measurements: Vec<LatencyMeasurement>,
    pub statistics: LatencyStatistics,
    pub passed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatencyStatistics {
    pub p50_ms: u64,
    pub p95_ms: u64,
    pub p99_ms: u64,
    pub mean_ms: f64,
    pub std_dev_ms: f64,
    pub sample_count: usize,
}
```

## Error Handling

### Measurement Failures

- **Incomplete measurements**: Log warning and exclude from statistics
- **Clock synchronization issues**: Use monotonic clocks for duration measurement
- **Storage failures**: Graceful degradation with in-memory fallback

### Test Environment Issues

- **CI runner performance variation**: Multiple test runs with statistical validation
- **Resource contention**: Isolated test execution with resource monitoring
- **Model loading failures**: Retry mechanism with exponential backoff

## Testing Strategy

### Unit Tests

- Telemetry collection accuracy and precision
- Statistical calculation correctness
- Error handling for edge cases

### Integration Tests

- End-to-end latency measurement pipeline
- Performance test framework execution
- CI integration and build gating

### Performance Tests

- **Baseline Performance**: 100 iterations of 5-second clips
- **Model Comparison**: Latency across all supported models
- **Stress Testing**: Performance under high system load
- **Regression Testing**: Historical comparison and trend analysis

### Acceptance Tests

- P95 latency ≤ 3s validation
- CI performance test execution
- Automated regression detection
