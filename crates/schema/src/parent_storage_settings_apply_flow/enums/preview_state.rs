use serde::{Deserialize, Serialize};

use super::super::constants::{
    PARENT_STORAGE_PREVIEW_STATE_BUNDLE_CORRUPT,
    PARENT_STORAGE_PREVIEW_STATE_IMPORT_PREVIEW_PASSED,
    PARENT_STORAGE_PREVIEW_STATE_MANUAL_REQUIRED, PARENT_STORAGE_PREVIEW_STATE_PARTIAL_RESTORE,
    PARENT_STORAGE_PREVIEW_STATE_SCHEMA_UNSUPPORTED,
    PARENT_STORAGE_PREVIEW_STATE_TOMBSTONE_CONFLICT, PARENT_STORAGE_PREVIEW_STATE_WRONG_HOUSEHOLD,
    PARENT_STORAGE_PREVIEW_STATE_WRONG_KEY,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStoragePreviewState {
    #[serde(rename = "importPreviewPassed")]
    ImportPreviewPassed,
    #[serde(rename = "partialRestore")]
    PartialRestore,
    #[serde(rename = "wrongHousehold")]
    WrongHousehold,
    #[serde(rename = "wrongKey")]
    WrongKey,
    #[serde(rename = "schemaUnsupported")]
    SchemaUnsupported,
    #[serde(rename = "bundleCorrupt")]
    BundleCorrupt,
    #[serde(rename = "tombstoneConflict")]
    TombstoneConflict,
    #[serde(rename = "manualRequired")]
    ManualRequired,
}

impl ParentStoragePreviewState {
    pub fn as_str(&self) -> &'static str {
        const VALUES: &[&str] = &[
            PARENT_STORAGE_PREVIEW_STATE_IMPORT_PREVIEW_PASSED,
            PARENT_STORAGE_PREVIEW_STATE_PARTIAL_RESTORE,
            PARENT_STORAGE_PREVIEW_STATE_WRONG_HOUSEHOLD,
            PARENT_STORAGE_PREVIEW_STATE_WRONG_KEY,
            PARENT_STORAGE_PREVIEW_STATE_SCHEMA_UNSUPPORTED,
            PARENT_STORAGE_PREVIEW_STATE_BUNDLE_CORRUPT,
            PARENT_STORAGE_PREVIEW_STATE_TOMBSTONE_CONFLICT,
            PARENT_STORAGE_PREVIEW_STATE_MANUAL_REQUIRED,
        ];
        VALUES[*self as usize]
    }
}
