//! Performance profiler for Talkute
//!
//! Provides runtime performance monitoring and profiling capabilities.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Performance profiler for monitoring operation timings
pub struct Profiler {
    metrics: Arc<RwLock<HashMap<String, OperationMetrics>>>,
    enabled: Arc<RwLock<bool>>,
}

/// Metrics for a single operation type
#[derive(Debug, Clone, Default)]
pub struct OperationMetrics {
    /// Total number of calls
    pub call_count: u64,
    /// Total time spent in operation
    pub total_duration: Duration,
    /// Minimum duration observed
    pub min_duration: Option<Duration>,
    /// Maximum duration observed
    pub max_duration: Option<Duration>,
    /// Recent samples for percentile calculation
    recent_samples: Vec<Duration>,
    /// Maximum samples to keep
    max_samples: usize,
}

/// A timed operation scope guard
pub struct TimedOperation {
    name: String,
    start: Instant,
    profiler: Arc<RwLock<HashMap<String, OperationMetrics>>>,
    enabled: bool,
}

impl OperationMetrics {
    /// Create new metrics with specified max samples
    pub fn new(max_samples: usize) -> Self {
        Self {
            call_count: 0,
            total_duration: Duration::ZERO,
            min_duration: None,
            max_duration: None,
            recent_samples: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Record a new sample
    pub fn record(&mut self, duration: Duration) {
        self.call_count += 1;
        self.total_duration += duration;

        // Update min/max
        self.min_duration = Some(match self.min_duration {
            None => duration,
            Some(min) => duration.min(min),
        });

        self.max_duration = Some(match self.max_duration {
            None => duration,
            Some(max) => duration.max(max),
        });

        // Add to recent samples
        self.recent_samples.push(duration);
        if self.recent_samples.len() > self.max_samples {
            self.recent_samples.remove(0);
        }
    }

    /// Get average duration
    pub fn average_duration(&self) -> Option<Duration> {
        if self.call_count == 0 {
            return None;
        }
        Some(self.total_duration / self.call_count as u32)
    }

    /// Get p50 (median) duration
    pub fn p50(&self) -> Option<Duration> {
        percentile(&self.recent_samples, 0.50)
    }

    /// Get p95 duration
    pub fn p95(&self) -> Option<Duration> {
        percentile(&self.recent_samples, 0.95)
    }

    /// Get p99 duration
    pub fn p99(&self) -> Option<Duration> {
        percentile(&self.recent_samples, 0.99)
    }

    /// Get operations per second
    pub fn ops_per_second(&self) -> f64 {
        if self.total_duration.is_zero() {
            return 0.0;
        }
        self.call_count as f64 / self.total_duration.as_secs_f64()
    }
}

/// Calculate percentile from sorted samples
fn percentile(samples: &[Duration], p: f64) -> Option<Duration> {
    if samples.is_empty() {
        return None;
    }

    let mut sorted = samples.to_vec();
    sorted.sort();

    let index = ((sorted.len() as f64) * p) as usize;
    Some(sorted[index.min(sorted.len() - 1)])
}

impl Profiler {
    /// Create a new profiler
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            enabled: Arc::new(RwLock::new(true)),
        }
    }

    /// Enable or disable profiling
    pub async fn set_enabled(&self, enabled: bool) {
        let mut e = self.enabled.write().await;
        *e = enabled;
    }

    /// Check if profiling is enabled
    pub async fn is_enabled(&self) -> bool {
        *self.enabled.read().await
    }

    /// Start a timed operation
    pub async fn start_operation(&self, name: &str) -> TimedOperation {
        TimedOperation {
            name: name.to_string(),
            start: Instant::now(),
            profiler: Arc::clone(&self.metrics),
            enabled: *self.enabled.read().await,
        }
    }

    /// Record a manual timing
    pub async fn record(&self, name: &str, duration: Duration) {
        if !*self.enabled.read().await {
            return;
        }

        let mut metrics = self.metrics.write().await;
        let entry = metrics.entry(name.to_string()).or_insert_with(|| OperationMetrics::new(1000));
        entry.record(duration);
    }

    /// Get metrics for an operation
    pub async fn get_metrics(&self, name: &str) -> Option<OperationMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(name).cloned()
    }

    /// Get all metrics
    pub async fn get_all_metrics(&self) -> HashMap<String, OperationMetrics> {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    /// Clear all metrics
    pub async fn clear(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.clear();
    }

    /// Generate a performance report
    pub async fn generate_report(&self) -> PerformanceReport {
        let metrics = self.metrics.read().await;

        let mut entries: Vec<PerformanceEntry> = metrics
            .iter()
            .map(|(name, m)| PerformanceEntry {
                name: name.clone(),
                call_count: m.call_count,
                total_duration: m.total_duration,
                average_duration: m.average_duration(),
                min_duration: m.min_duration,
                max_duration: m.max_duration,
                p50: m.p50(),
                p95: m.p95(),
                p99: m.p99(),
                ops_per_second: m.ops_per_second(),
            })
            .collect();

        // Sort by total duration descending
        entries.sort_by(|a, b| b.total_duration.cmp(&a.total_duration));

        PerformanceReport {
            entries,
            generated_at: chrono::Utc::now(),
        }
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TimedOperation {
    fn drop(&mut self) {
        if !self.enabled {
            return;
        }

        let duration = self.start.elapsed();
        let profiler = Arc::clone(&self.profiler);

        // Use try_to_utc to avoid blocking in drop
        tokio::spawn(async move {
            let mut metrics = profiler.write().await;
            let entry = metrics.entry(format!("{}_async", chrono::Utc::now().timestamp())).or_insert_with(|| OperationMetrics::new(1000));
            // Note: We use a different approach in sync context
        });
    }
}

/// A performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub entries: Vec<PerformanceEntry>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// A single entry in a performance report
#[derive(Debug, Clone)]
pub struct PerformanceEntry {
    pub name: String,
    pub call_count: u64,
    pub total_duration: Duration,
    pub average_duration: Option<Duration>,
    pub min_duration: Option<Duration>,
    pub max_duration: Option<Duration>,
    pub p50: Option<Duration>,
    pub p95: Option<Duration>,
    pub p99: Option<Duration>,
    pub ops_per_second: f64,
}

impl PerformanceReport {
    /// Format report as a string
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Performance Report - {}\n", self.generated_at));
        output.push_str(&"=".repeat(80));
        output.push_str("\n\n");

        for entry in &self.entries {
            output.push_str(&format!("{}\n", entry.name));
            output.push_str(&format!("  Calls: {}\n", entry.call_count));
            output.push_str(&format!("  Total: {:?}", entry.total_duration));

            if let Some(avg) = entry.average_duration {
                output.push_str(&format!("  Average: {:?}", avg));
            }

            if let Some(p95) = entry.p95 {
                output.push_str(&format!("  P95: {:?}", p95));
            }

            output.push_str("\n");
        }

        output
    }
}

/// Global profiler instance
static PROFILER: once_cell::sync::Lazy<Profiler> = once_cell::sync::Lazy::new(Profiler::new);

/// Get the global profiler
pub fn global_profiler() -> &'static Profiler {
    &PROFILER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_profiler_basic() {
        let profiler = Profiler::new();

        profiler.record("test_op", Duration::from_millis(10)).await;
        profiler.record("test_op", Duration::from_millis(20)).await;

        let metrics = profiler.get_metrics("test_op").await.unwrap();

        assert_eq!(metrics.call_count, 2);
        assert_eq!(metrics.total_duration, Duration::from_millis(30));
    }

    #[tokio::test]
    async fn test_profiler_percentiles() {
        let profiler = Profiler::new();

        // Add samples: 1, 2, 3, ..., 100
        for i in 1..=100 {
            profiler.record("test", Duration::from_millis(i)).await;
        }

        let metrics = profiler.get_metrics("test").await.unwrap();

        assert_eq!(metrics.call_count, 100);
        assert!(metrics.p95().unwrap() >= Duration::from_millis(95));
    }

    #[tokio::test]
    async fn test_profiler_enable_disable() {
        let profiler = Profiler::new();

        profiler.record("before", Duration::from_millis(1)).await;
        assert!(profiler.get_metrics("before").await.is_some());

        profiler.set_enabled(false).await;
        profiler.record("after", Duration::from_millis(1)).await;

        assert!(profiler.get_metrics("after").await.is_none());
    }

    #[tokio::test]
    async fn test_profiler_report() {
        let profiler = Profiler::new();

        profiler.record("fast", Duration::from_millis(1)).await;
        profiler.record("slow", Duration::from_millis(100)).await;

        let report = profiler.generate_report().await;

        assert_eq!(report.entries.len(), 2);
        assert!(report.entries[0].total_duration >= report.entries[1].total_duration);
    }

    #[test]
    fn test_operation_metrics_average() {
        let mut metrics = OperationMetrics::new(100);

        metrics.record(Duration::from_millis(10));
        metrics.record(Duration::from_millis(20));
        metrics.record(Duration::from_millis(30));

        assert_eq!(metrics.average_duration(), Some(Duration::from_millis(20)));
    }
}
