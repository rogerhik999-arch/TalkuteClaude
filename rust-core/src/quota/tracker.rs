//! Usage quota tracking

use crate::error::{Result, StorageError};
use crate::storage::database::Database;
use chrono::{Datelike, Duration, Utc};
use uuid::Uuid;

/// Usage quota service
pub struct UsageQuotaService {
    db: Database,
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
