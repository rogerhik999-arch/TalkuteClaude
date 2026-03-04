//! Device profile management

use crate::error::{Result, StorageError};
use crate::storage::database::Database;
use crate::storage::models::DeviceProfile;
use uuid::Uuid;

/// Device profile service
pub struct DeviceProfileService {
    db: Database,
}

impl DeviceProfileService {
    /// Create a new device profile service
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Get or create the device profile
    pub fn get_or_create(&self) -> Result<DeviceProfile> {
        // Try to get existing profile
        if let Some(profile) = self.get()? {
            return Ok(profile);
        }

        // Create new profile
        let device_id = Uuid::new_v4().to_string();
        let profile = DeviceProfile::new(device_id.clone());
        self.insert(&profile)?;

        Ok(profile)
    }

    /// Get the device profile
    pub fn get(&self) -> Result<Option<DeviceProfile>> {
        let conn = self.db.connection();

        let profile = conn.query_row(
            "SELECT device_id, created_at, last_active_at, preferred_language,
                    voice_speed_preference, auto_punctuation_enabled, filler_removal_enabled,
                    self_correction_enabled, crash_reporting_enabled
             FROM device_profiles
             LIMIT 1",
            [],
            |row| {
                Ok(DeviceProfile {
                    device_id: row.get(0)?,
                    created_at: row.get(1)?,
                    last_active_at: row.get(2)?,
                    preferred_language: row.get(3)?,
                    voice_speed_preference: row.get(4)?,
                    auto_punctuation_enabled: row.get::<_, i32>(5)? != 0,
                    filler_removal_enabled: row.get::<_, i32>(6)? != 0,
                    self_correction_enabled: row.get::<_, i32>(7)? != 0,
                    crash_reporting_enabled: row.get::<_, i32>(8)? != 0,
                })
            },
        ).ok();

        Ok(profile)
    }

    /// Insert a new device profile
    fn insert(&self, profile: &DeviceProfile) -> Result<()> {
        let conn = self.db.connection();

        conn.execute(
            "INSERT INTO device_profiles (
                device_id, created_at, last_active_at, preferred_language,
                voice_speed_preference, auto_punctuation_enabled, filler_removal_enabled,
                self_correction_enabled, crash_reporting_enabled
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                profile.device_id,
                profile.created_at.to_rfc3339(),
                profile.last_active_at.to_rfc3339(),
                profile.preferred_language,
                profile.voice_speed_preference,
                if profile.auto_punctuation_enabled { 1 } else { 0 },
                if profile.filler_removal_enabled { 1 } else { 0 },
                if profile.self_correction_enabled { 1 } else { 0 },
                if profile.crash_reporting_enabled { 1 } else { 0 },
            ],
        )
        .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    /// Update the device profile
    pub fn update(&self, profile: &DeviceProfile) -> Result<()> {
        let conn = self.db.connection();

        conn.execute(
            "UPDATE device_profiles SET
                last_active_at = ?1,
                preferred_language = ?2,
                voice_speed_preference = ?3,
                auto_punctuation_enabled = ?4,
                filler_removal_enabled = ?5,
                self_correction_enabled = ?6,
                crash_reporting_enabled = ?7
             WHERE device_id = ?8",
            rusqlite::params![
                profile.last_active_at.to_rfc3339(),
                profile.preferred_language,
                profile.voice_speed_preference,
                if profile.auto_punctuation_enabled { 1 } else { 0 },
                if profile.filler_removal_enabled { 1 } else { 0 },
                if profile.self_correction_enabled { 1 } else { 0 },
                if profile.crash_reporting_enabled { 1 } else { 0 },
                profile.device_id,
            ],
        )
        .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }
}
