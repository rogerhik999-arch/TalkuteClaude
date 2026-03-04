//! Usage quota tracking

use crate::error::{Result, StorageError};
use crate::storage::database::Database;
use chrono::{Datelike, Duration, Utc};
use uuid::Uuid;

/// Default weekly word limit
pub const WEEKLY_LIMIT: i32 = 4000;

/// Warning threshold percentage (80%)
pub const WARNING_THRESHOLD_PERCENTAGE: f64 = 80.0;

/// Usage quota service
pub struct UsageQuotaService {
    db: Database,
}

/// Quota tracker for simplified quota operations
pub struct QuotaTracker {
    db: Database,
    device_id: String,
}

impl QuotaTracker {
    /// Create a new quota tracker
    pub fn new(db: Database) -> Result<Self> {
        Ok(Self {
            db,
            device_id: "default".to_string(),
        })
    }

    /// Check if quota is available for the given word count
    pub async fn check_quota(&self, words_needed: i32) -> Result<bool> {
        let usage = self.get_weekly_usage().await?;
        Ok(usage + words_needed <= WEEKLY_LIMIT)
    }

    /// Get the current weekly usage
    pub async fn get_weekly_usage(&self) -> Result<i32> {
        let conn = self.db.connection();
        let usage = conn.query_row(
            "SELECT COALESCE(SUM(words_used_this_week), 0) FROM usage_quotas WHERE device_id = ?1",
            [&self.device_id],
            |row| row.get::<_, i32>(0),
        ).unwrap_or(0);
        Ok(usage)
    }

    /// Add words to the quota usage
    pub async fn add_words(&self, word_count: i32) -> Result<()> {
        // Ensure device profile exists first (for foreign key constraint)
        let conn = self.db.connection();
        conn.execute(
            "INSERT OR IGNORE INTO device_profiles (device_id, created_at, last_active_at)
             VALUES (?1, datetime('now'), datetime('now'))",
            [&self.device_id],
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        // Check if quota entry exists
        let exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM usage_quotas WHERE device_id = ?1",
            [&self.device_id],
            |row| row.get(0),
        ).unwrap_or(false);

        if !exists {
            let now = Utc::now();
            let today = now.date_naive();
            let weekday = today.weekday().num_days_from_monday() as i64;
            let week_start = today - Duration::days(weekday);

            conn.execute(
                "INSERT INTO usage_quotas (
                    quota_id, device_id, current_week_start,
                    words_used_this_week, weekly_limit, last_reset_at, total_words_all_time
                ) VALUES (?1, ?2, ?3, 0, ?4, ?5, 0)",
                rusqlite::params![
                    uuid::Uuid::new_v4().to_string(),
                    self.device_id,
                    week_start.to_string(),
                    WEEKLY_LIMIT,
                    now.to_rfc3339(),
                ],
            ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;
        }

        conn.execute(
            "UPDATE usage_quotas SET
                words_used_this_week = words_used_this_week + ?1,
                total_words_all_time = total_words_all_time + ?1
             WHERE device_id = ?2",
            rusqlite::params![word_count, self.device_id],
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    /// Get the remaining quota for this week
    pub async fn get_remaining_quota(&self) -> Result<i32> {
        let usage = self.get_weekly_usage().await?;
        Ok((WEEKLY_LIMIT - usage).max(0))
    }

    /// Get the percentage of quota used
    pub async fn get_percentage_used(&self) -> Result<f64> {
        let usage = self.get_weekly_usage().await?;
        Ok((usage as f64 / WEEKLY_LIMIT as f64) * 100.0)
    }

    /// Get the weekly limit
    pub fn get_weekly_limit(&self) -> i32 {
        WEEKLY_LIMIT
    }

    /// Check if usage is at warning threshold (80%+)
    pub async fn is_at_warning_threshold(&self) -> Result<bool> {
        let percentage = self.get_percentage_used().await?;
        Ok(percentage >= WARNING_THRESHOLD_PERCENTAGE)
    }
}

impl UsageQuotaService {
    /// Create a new usage quota service
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Check if the quota is available for the given word count
    pub fn check_available(&self, device_id: &str, words_needed: i32) -> Result<bool> {
        let quota = self.get_or_create(device_id)?;
        Ok(quota.words_used_this_week + words_needed <= quota.weekly_limit)
    }

    /// Get the current quota for a device
    pub fn get(&self, device_id: &str) -> Result<Option<UsageQuota>> {
        let conn = self.db.connection();

        let quota = conn.query_row(
            "SELECT quota_id, device_id, current_week_start,
                    words_used_this_week, weekly_limit, last_reset_at, total_words_all_time
             FROM usage_quotas
             WHERE device_id = ?1",
            [device_id],
            |row| {
                Ok(UsageQuota {
                    quota_id: row.get(0)?,
                    device_id: row.get(1)?,
                    current_week_start: row.get(2)?,
                    words_used_this_week: row.get(3)?,
                    weekly_limit: row.get(4)?,
                    last_reset_at: row.get(5)?,
                    total_words_all_time: row.get(6)?,
                })
            },
        ).ok();

        Ok(quota)
    }

    /// Get or create quota for a device
    fn get_or_create(&self, device_id: &str) -> Result<UsageQuota> {
        if let Some(quota) = self.get(device_id)? {
            // Check if we need to reset the quota (weekly reset)
            if self.should_reset(&quota) {
                return self.reset(device_id);
            }
            return Ok(quota);
        }

        self.create_new(device_id)
    }

    /// Create a new quota for a device
    fn create_new(&self, device_id: &str) -> Result<UsageQuota> {
        let quota = UsageQuota::new(Uuid::new_v4().to_string(), device_id.to_string());
        let conn = self.db.connection();

        conn.execute(
            "INSERT INTO usage_quotas (
                quota_id, device_id, current_week_start,
                words_used_this_week, weekly_limit, last_reset_at, total_words_all_time
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                quota.quota_id,
                quota.device_id,
                quota.current_week_start,
                quota.words_used_this_week,
                quota.weekly_limit,
                quota.last_reset_at,
                quota.total_words_all_time,
            ],
        )
        .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(quota)
    }

    /// Check if quota should be reset (new week)
    fn should_reset(&self, quota: &UsageQuota) -> bool {
        let current_week_start = chrono::NaiveDate::parse_from_str(&quota.current_week_start, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        let today = Utc::now().date_naive();

        // Reset on Monday (.weekday().num_days_from_monday() == 0)
        today >= current_week_start + Duration::days(7)
    }

    /// Reset quota for a device
    fn reset(&self, device_id: &str) -> Result<UsageQuota> {
        let quota = self.get(device_id)
            .ok()
            .flatten()
            .unwrap_or_else(|| UsageQuota::new(Uuid::new_v4().to_string(), device_id.to_string()));

        let now = Utc::now();
        let today = now.date_naive();
        // Get Monday of current week
        let weekday = today.weekday().num_days_from_monday() as i64;
        let week_start = today - Duration::days(weekday);

        let conn = self.db.connection();

        conn.execute(
            "UPDATE usage_quotas SET
                current_week_start = ?1,
                words_used_this_week = 0,
                last_reset_at = ?2
             WHERE device_id = ?3",
            rusqlite::params![
                week_start.to_string(),
                now.to_rfc3339(),
                device_id,
            ],
        )
        .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(UsageQuota {
            current_week_start: week_start.to_string(),
            words_used_this_week: 0,
            ..quota
        })
    }

    /// Add words to quota usage
    pub fn add_words(&self, device_id: &str, word_count: i32) -> Result<()> {
        let conn = self.db.connection();

        conn.execute(
            "UPDATE usage_quotas SET
                words_used_this_week = words_used_this_week + ?1,
                total_words_all_time = total_words_all_time + ?1
             WHERE device_id = ?2",
            rusqlite::params![word_count, device_id],
        )
        .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    /// Get quota usage info
    pub fn get_usage_info(&self, device_id: &str) -> Result<QuotaInfo> {
        let quota = self.get_or_create(device_id)?;
        let percentage = (quota.words_used_this_week as f64 / quota.weekly_limit as f64) * 100.0;

        Ok(QuotaInfo {
            words_used_this_week: quota.words_used_this_week,
            weekly_limit: quota.weekly_limit,
            percentage_used: percentage,
        })
    }
}

/// Usage quota information
#[derive(Debug, Clone)]
pub struct UsageQuota {
    pub quota_id: String,
    pub device_id: String,
    pub current_week_start: String,
    pub words_used_this_week: i32,
    pub weekly_limit: i32,
    pub last_reset_at: String,
    pub total_words_all_time: i32,
}

/// Quota usage information for UI display
#[derive(Debug, Clone)]
pub struct QuotaInfo {
    pub words_used_this_week: i32,
    pub weekly_limit: i32,
    pub percentage_used: f64,
}

impl UsageQuota {
    fn new(quota_id: String, device_id: String) -> Self {
        let now = Utc::now();
        let today = now.date_naive();
        // Get Monday of current week
        let weekday = today.weekday().num_days_from_monday() as i64;
        let week_start = today - Duration::days(weekday);

        Self {
            quota_id,
            device_id,
            current_week_start: week_start.to_string(),
            words_used_this_week: 0,
            weekly_limit: 4000,
            last_reset_at: now.to_rfc3339(),
            total_words_all_time: 0,
        }
    }
}
