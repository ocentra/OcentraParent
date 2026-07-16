use ocentra_schema::parent_storage_settings_apply_flow as contracts;

pub(super) fn summary_for_mode(
    mode: contracts::ParentStorageModeLabel,
    ui_state: contracts::ParentStorageUiState,
) -> &'static str {
    match (mode, ui_state) {
        (contracts::ParentStorageModeLabel::LocalOnly, _) => {
            "Local-only storage remains the current custody path."
        }
        (contracts::ParentStorageModeLabel::LocalPlusEncryptedBackup, _) => {
            "Parent-owned local backup remains explicit and encrypted."
        }
        (contracts::ParentStorageModeLabel::LocalPlusEncryptedProviderSync, _) => {
            "Provider sync stays encrypted and parent-owned."
        }
        (contracts::ParentStorageModeLabel::ProviderDisconnected, _) => {
            "Provider is disconnected; existing files may remain until separate delete proof succeeds."
        }
        (contracts::ParentStorageModeLabel::ProviderError, _) => {
            "Provider failure is explicit; no success-looking state is shown."
        }
        (contracts::ParentStorageModeLabel::ManualRequired, _) => {
            "Manual proof is required before a safe apply or delete step proceeds."
        }
        (
            contracts::ParentStorageModeLabel::Disabled,
            contracts::ParentStorageUiState::SyncDisabled,
        ) => "Provider sync is intentionally disabled.",
        (contracts::ParentStorageModeLabel::Disabled, _) => "Provider storage is not configured.",
    }
}
