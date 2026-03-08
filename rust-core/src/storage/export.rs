//! Data export functionality for Talkute
//!
//! Provides export capabilities for user data including:
//! - Personal dictionary entries
//! - Transcription history
//! - Application settings
//! - Usage statistics

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs::File;
use std::io::Write;

use crate::error::{Result, StorageError};
use crate::storage::database::Database;
use crate::storage::models::{PersonalDictionaryEntry, TranscriptionHistory};

/// Export format types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    /// JSON format (default)
    Json,
    /// CSV format (for spreadsheets)
    Csv,
    /// Plain text format
    Txt,
}

/// Export data container
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    /// Export metadata
    pub metadata: ExportMetadata,
    /// Personal dictionary entries
    pub dictionary: Vec<DictionaryEntryExport>,
    /// Transcription history
    pub history: Vec<HistoryEntryExport>,
    /// Application settings
    pub settings: SettingsExport,
}

/// Export metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Export version for compatibility
    pub version: String,
    /// Export timestamp
    pub exported_at: String,
    /// Application version that created the export
    pub app_version: String,
}

/// Dictionary entry for export
#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryEntryExport {
    pub term: String,
    pub pronunciation: Option<String>,
    pub definition: Option<String>,
    pub category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// History entry for export
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntryExport {
    pub id: String,
    pub raw_text: String,
    pub polished_text: Option<String>,
    pub translated_text: Option<String>,
    pub language: String,
    pub context: Option<String>,
    pub created_at: String,
}

/// Settings for export
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsExport {
    pub input_language: String,
    pub translation_language: String,
    pub auto_translate: bool,
    pub auto_process: bool,
    pub filler_removal: bool,
    pub context_aware: bool,
    pub push_to_talk: bool,
    pub noise_cancellation: bool,
    pub crash_reporting: bool,
    pub analytics: bool,
}

/// Data exporter
pub struct DataExporter {
    db: Database,
}

impl DataExporter {
    /// Create a new data exporter
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Export all data to the specified format
    pub async fn export_all(&self, format: ExportFormat, path: &Path) -> Result<()> {
        let data = self.gather_export_data().await?;

        match format {
            ExportFormat::Json => self.export_json(&data, path)?,
            ExportFormat::Csv => self.export_csv(&data, path)?,
            ExportFormat::Txt => self.export_txt(&data, path)?,
        }

        Ok(())
    }

    /// Export only dictionary entries
    pub async fn export_dictionary(&self, format: ExportFormat, path: &Path) -> Result<()> {
        let entries = self.get_dictionary_entries().await?;

        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&entries)
                    .map_err(|e| StorageError::ExportFailed(e.to_string()))?;
                self.write_file(path, &json)?;
            }
            ExportFormat::Csv => {
                let csv = self.dictionary_to_csv(&entries)?;
                self.write_file(path, &csv)?;
            }
            ExportFormat::Txt => {
                let txt = self.dictionary_to_txt(&entries);
                self.write_file(path, &txt)?;
            }
        }

        Ok(())
    }

    /// Export only transcription history
    pub async fn export_history(&self, format: ExportFormat, path: &Path) -> Result<()> {
        let entries = self.get_history_entries().await?;

        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&entries)
                    .map_err(|e| StorageError::ExportFailed(e.to_string()))?;
                self.write_file(path, &json)?;
            }
            ExportFormat::Csv => {
                let csv = self.history_to_csv(&entries)?;
                self.write_file(path, &csv)?;
            }
            ExportFormat::Txt => {
                let txt = self.history_to_txt(&entries);
                self.write_file(path, &txt)?;
            }
        }

        Ok(())
    }

    /// Gather all export data
    async fn gather_export_data(&self) -> Result<ExportData> {
        let dictionary = self.get_dictionary_entries().await?;
        let history = self.get_history_entries().await?;
        let settings = self.get_settings().await?;

        Ok(ExportData {
            metadata: ExportMetadata {
                version: "1.0".to_string(),
                exported_at: chrono::Utc::now().to_rfc3339(),
                app_version: env!("CARGO_PKG_VERSION").to_string(),
            },
            dictionary,
            history,
            settings,
        })
    }

    /// Get dictionary entries for export
    async fn get_dictionary_entries(&self) -> Result<Vec<DictionaryEntryExport>> {
        // In a real implementation, this would query the database
        Ok(Vec::new())
    }

    /// Get history entries for export
    async fn get_history_entries(&self) -> Result<Vec<HistoryEntryExport>> {
        // In a real implementation, this would query the database
        Ok(Vec::new())
    }

    /// Get settings for export
    async fn get_settings(&self) -> Result<SettingsExport> {
        // In a real implementation, this would read from settings storage
        Ok(SettingsExport {
            input_language: "en".to_string(),
            translation_language: "zh".to_string(),
            auto_translate: false,
            auto_process: true,
            filler_removal: true,
            context_aware: true,
            push_to_talk: false,
            noise_cancellation: true,
            crash_reporting: false,
            analytics: false,
        })
    }

    /// Export to JSON format
    fn export_json(&self, data: &ExportData, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| StorageError::ExportFailed(e.to_string()))?;
        self.write_file(path, &json)
    }

    /// Export to CSV format
    fn export_csv(&self, data: &ExportData, path: &Path) -> Result<()> {
        let mut csv = String::new();

        // Dictionary section
        csv.push_str("=== DICTIONARY ===\n");
        csv.push_str("Term,Pronunciation,Definition,Category,Created At,Updated At\n");
        for entry in &data.dictionary {
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                entry.term,
                entry.pronunciation.as_deref().unwrap_or(""),
                entry.definition.as_deref().unwrap_or(""),
                entry.category.as_deref().unwrap_or(""),
                entry.created_at,
                entry.updated_at
            ));
        }

        csv.push_str("\n=== HISTORY ===\n");
        csv.push_str("ID,Raw Text,Polished Text,Translated Text,Language,Context,Created At\n");
        for entry in &data.history {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                entry.id,
                entry.raw_text.replace(',', "\\,"),
                entry.polished_text.as_deref().unwrap_or("").replace(',', "\\,"),
                entry.translated_text.as_deref().unwrap_or("").replace(',', "\\,"),
                entry.language,
                entry.context.as_deref().unwrap_or(""),
                entry.created_at
            ));
        }

        self.write_file(path, &csv)
    }

    /// Export to plain text format
    fn export_txt(&self, data: &ExportData, path: &Path) -> Result<()> {
        let mut txt = String::new();

        txt.push_str(&format!("Talkute Data Export\n"));
        txt.push_str(&format!("Exported: {}\n", data.metadata.exported_at));
        txt.push_str(&format!("Version: {}\n\n", data.metadata.version));

        txt.push_str("=== PERSONAL DICTIONARY ===\n\n");
        for entry in &data.dictionary {
            txt.push_str(&format!("Term: {}\n", entry.term));
            if let Some(ref pron) = entry.pronunciation {
                txt.push_str(&format!("Pronunciation: {}\n", pron));
            }
            if let Some(ref def) = entry.definition {
                txt.push_str(&format!("Definition: {}\n", def));
            }
            if let Some(ref cat) = entry.category {
                txt.push_str(&format!("Category: {}\n", cat));
            }
            txt.push_str(&format!("Created: {}\n\n", entry.created_at));
        }

        txt.push_str("\n=== TRANSCRIPTION HISTORY ===\n\n");
        for entry in &data.history {
            txt.push_str(&format!("Date: {}\n", entry.created_at));
            txt.push_str(&format!("Language: {}\n", entry.language));
            txt.push_str(&format!("Raw: {}\n", entry.raw_text));
            if let Some(ref polished) = entry.polished_text {
                txt.push_str(&format!("Polished: {}\n", polished));
            }
            if let Some(ref translated) = entry.translated_text {
                txt.push_str(&format!("Translated: {}\n", translated));
            }
            txt.push_str("\n---\n\n");
        }

        self.write_file(path, &txt)
    }

    /// Convert dictionary entries to CSV
    fn dictionary_to_csv(&self, entries: &[DictionaryEntryExport]) -> Result<String> {
        let mut csv = String::from("Term,Pronunciation,Definition,Category,Created At,Updated At\n");
        for entry in entries {
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                entry.term,
                entry.pronunciation.as_deref().unwrap_or(""),
                entry.definition.as_deref().unwrap_or(""),
                entry.category.as_deref().unwrap_or(""),
                entry.created_at,
                entry.updated_at
            ));
        }
        Ok(csv)
    }

    /// Convert dictionary entries to plain text
    fn dictionary_to_txt(&self, entries: &[DictionaryEntryExport]) -> String {
        let mut txt = String::from("Personal Dictionary Export\n\n");
        for entry in entries {
            txt.push_str(&format!("Term: {}\n", entry.term));
            if let Some(ref pron) = entry.pronunciation {
                txt.push_str(&format!("Pronunciation: {}\n", pron));
            }
            if let Some(ref def) = entry.definition {
                txt.push_str(&format!("Definition: {}\n", def));
            }
            txt.push_str("\n");
        }
        txt
    }

    /// Convert history entries to CSV
    fn history_to_csv(&self, entries: &[HistoryEntryExport]) -> Result<String> {
        let mut csv = String::from("ID,Raw Text,Polished Text,Language,Created At\n");
        for entry in entries {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                entry.id,
                entry.raw_text.replace(',', "\\,"),
                entry.polished_text.as_deref().unwrap_or("").replace(',', "\\,"),
                entry.language,
                entry.created_at
            ));
        }
        Ok(csv)
    }

    /// Convert history entries to plain text
    fn history_to_txt(&self, entries: &[HistoryEntryExport]) -> String {
        let mut txt = String::from("Transcription History Export\n\n");
        for entry in entries {
            txt.push_str(&format!("Date: {}\n", entry.created_at));
            txt.push_str(&format!("Language: {}\n", entry.language));
            txt.push_str(&format!("Text: {}\n\n", entry.raw_text));
        }
        txt
    }

    /// Write content to file
    fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        let mut file = File::create(path)
            .map_err(|e| StorageError::ExportFailed(e.to_string()))?;
        file.write_all(content.as_bytes())
            .map_err(|e| StorageError::ExportFailed(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_default() {
        let format = ExportFormat::Json;
        assert_eq!(format, ExportFormat::Json);
    }

    #[test]
    fn test_settings_export_serialization() {
        let settings = SettingsExport {
            input_language: "en".to_string(),
            translation_language: "zh".to_string(),
            auto_translate: false,
            auto_process: true,
            filler_removal: true,
            context_aware: true,
            push_to_talk: false,
            noise_cancellation: true,
            crash_reporting: false,
            analytics: false,
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"input_language\":\"en\""));
        assert!(json.contains("\"auto_process\":true"));
    }
}
