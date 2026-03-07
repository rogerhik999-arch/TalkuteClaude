//! Quota management for Talkute
//!
//! Provides usage tracking and quota enforcement with grace periods.

use chrono::{DateTime, Utc, Datelike, Duration};
use serde::{Deserialize, Serialize};

/// Quota configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaConfig {
    /// Maximum words allowed per week
    pub weekly_limit: u32,
    /// Grace period percentage (e.g., 10% overage allowed)
    pub grace_percentage: u8,
    /// Whether quota is enforced
    pub enabled: bool,
}

impl Default for QuotaConfig {
    fn default() -> Self {
        Self {
            weekly_limit: 4000,
            grace_percentage: 10,
            enabled: true,
        }
    }
}

/// Current quota status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaStatus {
    /// Words used this week
    pub words_used: u32,
    /// Weekly limit
    pub weekly_limit: u32,
    /// Whether in grace period
    pub in_grace_period: bool,
    /// Whether quota is exceeded
    pub exceeded: bool,
    /// Remaining words (0 if exceeded)
    pub remaining: u32,
    /// Percentage used
    pub percentage_used: f32,
    /// Week start date
    pub week_start: DateTime<Utc>,
    /// Week end date
    pub week_end: DateTime<Utc>,
}

impl QuotaStatus {
    /// Create a new quota status
    pub fn new(words_used: u32, config: &QuotaConfig) -> Self {
        let grace_limit = config.weekly_limit as f32 * (1.0 + config.grace_percentage as f32 / 100.0);
        let exceeded = words_used > grace_limit as u32;
        let in_grace_period = words_used > config.weekly_limit && !exceeded;
        let remaining = if exceeded {
            0
        } else {
            config.weekly_limit.saturating_sub(words_used)
        };
        let percentage_used = if config.weekly_limit > 0 {
            (words_used as f32 / config.weekly_limit as f32) * 100.0
        } else {
            0.0
        };

        let now = Utc::now();
        let week_start = now - Duration::days(now.weekday().num_days_from_monday() as i64);
        let week_end = week_start + Duration::days(7);

        Self {
            words_used,
            weekly_limit: config.weekly_limit,
            in_grace_period,
            exceeded,
            remaining,
            percentage_used: percentage_used.min(100.0),
            week_start,
            week_end,
        }
    }

    /// Check if quota is available for a given word count
    pub fn is_available(&self, words_needed: u32) -> bool {
        !self.exceeded && (self.remaining >= words_needed || self.in_grace_period)
    }
}

/// Quota manager for tracking and enforcing usage limits
pub struct QuotaManager {
    config: QuotaConfig,
    current_usage: u32,
}

impl QuotaManager {
    /// Create a new quota manager
    pub fn new(config: QuotaConfig) -> Self {
        Self {
            config,
            current_usage: 0,
        }
    }

    /// Get current quota status
    pub fn status(&self) -> QuotaStatus {
        QuotaStatus::new(self.current_usage, &self.config)
    }

    /// Check if quota is available for a given word count
    pub fn check_available(&self, words_needed: u32) -> bool {
        if !self.config.enabled {
            return true;
        }
        self.status().is_available(words_needed)
    }

    /// Add words to the current usage
    pub fn add_usage(&mut self, words: u32) {
        self.current_usage += words;
    }

    /// Reset usage (typically at week start)
    pub fn reset(&mut self) {
        self.current_usage = 0;
    }

    /// Update configuration
    pub fn set_config(&mut self, config: QuotaConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &QuotaConfig {
        &self.config
    }

    /// Get current usage
    pub fn current_usage(&self) -> u32 {
        self.current_usage
    }
}

impl Default for QuotaManager {
    fn default() -> Self {
        Self::new(QuotaConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_status_normal() {
        let config = QuotaConfig::default();
        let status = QuotaStatus::new(2000, &config);

        assert_eq!(status.words_used, 2000);
        assert!(!status.in_grace_period);
        assert!(!status.exceeded);
        assert_eq!(status.remaining, 2000);
        assert!((status.percentage_used - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_quota_status_grace_period() {
        let config = QuotaConfig {
            weekly_limit: 4000,
            grace_percentage: 10,
            enabled: true,
        };
        let status = QuotaStatus::new(4100, &config);

        assert!(status.in_grace_period);
        assert!(!status.exceeded);
    }

    #[test]
    fn test_quota_status_exceeded() {
        let config = QuotaConfig {
            weekly_limit: 4000,
            grace_percentage: 10,
            enabled: true,
        };
        // Grace limit is 4000 * 1.1 = 4400
        let status = QuotaStatus::new(4500, &config);

        assert!(!status.in_grace_period);
        assert!(status.exceeded);
        assert_eq!(status.remaining, 0);
    }

    #[test]
    fn test_quota_manager_check_available() {
        let mut manager = QuotaManager::default();
        manager.add_usage(3900);

        // remaining = 100, in_grace_period = false (3900 <= 4000)
        assert!(manager.check_available(100)); // Under remaining
        assert!(!manager.check_available(200)); // Exceeds remaining, not in grace

        // Now test grace period
        manager.add_usage(200); // usage = 4100
        // remaining = 0, but in_grace_period = true (4000 < 4100 < 4400)
        assert!(manager.check_available(100)); // Still available in grace
    }

    #[test]
    fn test_quota_manager_disabled() {
        let config = QuotaConfig {
            weekly_limit: 100,
            grace_percentage: 10,
            enabled: false,
        };
        let manager = QuotaManager::new(config);

        // Even with 0 usage, disabled quota should always allow
        assert!(manager.check_available(10000));
    }

    #[test]
    fn test_quota_manager_reset() {
        let mut manager = QuotaManager::default();
        manager.add_usage(500);
        assert_eq!(manager.current_usage(), 500);

        manager.reset();
        assert_eq!(manager.current_usage(), 0);
    }
}