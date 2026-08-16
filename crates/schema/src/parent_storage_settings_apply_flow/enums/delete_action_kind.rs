use serde::{Deserialize, Serialize};

use super::super::constants::{
    PARENT_STORAGE_DELETE_ACTION_KIND_GENERATED_REPORT,
    PARENT_STORAGE_DELETE_ACTION_KIND_LOCAL_CHILD_EVIDENCE,
    PARENT_STORAGE_DELETE_ACTION_KIND_OCENTRA_METADATA,
    PARENT_STORAGE_DELETE_ACTION_KIND_PARENT_PORTAL_CACHE,
    PARENT_STORAGE_DELETE_ACTION_KIND_PROVIDER_BACKUP_COPY,
    PARENT_STORAGE_DELETE_ACTION_KIND_SUPPORT_BUNDLE,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageDeleteActionKind {
    #[serde(rename = "delete-local-child-evidence")]
    LocalChildEvidence,
    #[serde(rename = "delete-parent-portal-cache")]
    ParentPortalCache,
    #[serde(rename = "delete-generated-report")]
    GeneratedReport,
    #[serde(rename = "delete-provider-backup-copy")]
    ProviderBackupCopy,
    #[serde(rename = "delete-support-bundle")]
    SupportBundle,
    #[serde(rename = "delete-ocentra-metadata")]
    OcentraMetadata,
}

impl ParentStorageDeleteActionKind {
    pub fn as_str(&self) -> &'static str {
        const VALUES: &[&str] = &[
            PARENT_STORAGE_DELETE_ACTION_KIND_LOCAL_CHILD_EVIDENCE,
            PARENT_STORAGE_DELETE_ACTION_KIND_PARENT_PORTAL_CACHE,
            PARENT_STORAGE_DELETE_ACTION_KIND_GENERATED_REPORT,
            PARENT_STORAGE_DELETE_ACTION_KIND_PROVIDER_BACKUP_COPY,
            PARENT_STORAGE_DELETE_ACTION_KIND_SUPPORT_BUNDLE,
            PARENT_STORAGE_DELETE_ACTION_KIND_OCENTRA_METADATA,
        ];
        VALUES[*self as usize]
    }
}
