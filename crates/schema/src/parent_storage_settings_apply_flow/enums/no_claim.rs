use serde::{Deserialize, Serialize};

use super::super::constants::{
    PARENT_STORAGE_NO_CLAIM_AUTO_APPLY, PARENT_STORAGE_NO_CLAIM_DELETE_DISCONNECT_COLLAPSE,
    PARENT_STORAGE_NO_CLAIM_DISCONNECT_DELETES_PROVIDER_DATA,
    PARENT_STORAGE_NO_CLAIM_LAN_OWNERSHIP, PARENT_STORAGE_NO_CLAIM_PORTAL_IMPLEMENTATION_READY,
    PARENT_STORAGE_NO_CLAIM_PROVIDER_RUNTIME_READY, PARENT_STORAGE_NO_CLAIM_TS_BUSINESS_OWNER,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageNoClaim {
    #[serde(rename = "no-portal-implementation-ready")]
    NoPortalImplementationReady,
    #[serde(rename = "no-provider-runtime-ready")]
    NoProviderRuntimeReady,
    #[serde(rename = "no-auto-apply")]
    NoAutoApply,
    #[serde(rename = "no-disconnect-deletes-provider-data")]
    NoDisconnectDeletesProviderData,
    #[serde(rename = "no-delete-disconnect-collapse")]
    NoDeleteDisconnectCollapse,
    #[serde(rename = "no-ts-business-owner")]
    NoTsBusinessOwner,
    #[serde(rename = "no-lan-ownership")]
    NoLanOwnership,
}

impl ParentStorageNoClaim {
    pub fn as_str(&self) -> &'static str {
        const VALUES: &[&str] = &[
            PARENT_STORAGE_NO_CLAIM_PORTAL_IMPLEMENTATION_READY,
            PARENT_STORAGE_NO_CLAIM_PROVIDER_RUNTIME_READY,
            PARENT_STORAGE_NO_CLAIM_AUTO_APPLY,
            PARENT_STORAGE_NO_CLAIM_DISCONNECT_DELETES_PROVIDER_DATA,
            PARENT_STORAGE_NO_CLAIM_DELETE_DISCONNECT_COLLAPSE,
            PARENT_STORAGE_NO_CLAIM_TS_BUSINESS_OWNER,
            PARENT_STORAGE_NO_CLAIM_LAN_OWNERSHIP,
        ];
        VALUES[*self as usize]
    }
}
