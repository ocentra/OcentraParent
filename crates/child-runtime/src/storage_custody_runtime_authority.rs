use std::{path::Path, sync::Arc};

use ocentra_storage_custody_core::{
    storage_custody::StorageCustodyEffectKind,
    storage_custody_effect_store::StorageCustodyEffectRecord,
};

use super::{
    ChildStorageCustodyAuthority, ChildStorageCustodyAuthorityError,
    ChildStorageCustodyAuthorityHandle, ManualRequiredChildStorageCustodyAuthority,
};

impl std::fmt::Debug for ChildStorageCustodyAuthorityHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ChildStorageCustodyAuthorityHandle")
            .field("source", &"opaque-current-authority")
            .finish()
    }
}

impl ChildStorageCustodyAuthorityHandle {
    pub fn manual_required() -> Self {
        Self::from_source(Arc::new(ManualRequiredChildStorageCustodyAuthority))
    }

    pub(crate) fn from_source(source: Arc<dyn ChildStorageCustodyAuthority>) -> Self {
        Self { source }
    }

    pub(super) fn household_id(&self) -> &str {
        self.source.household_id()
    }

    pub(super) fn child_profile_id(&self) -> &str {
        self.source.child_profile_id()
    }

    pub(super) fn target_device_id(&self) -> &str {
        self.source.target_device_id()
    }

    pub(super) fn authority_generation(&self) -> u64 {
        self.source.authority_generation()
    }

    pub(super) fn session_generation(&self) -> u64 {
        self.source.session_generation()
    }

    pub(super) fn validate_for(
        &self,
        effect: StorageCustodyEffectKind,
    ) -> Result<(), ChildStorageCustodyAuthorityError> {
        if self.household_id().trim().is_empty()
            || self.child_profile_id().trim().is_empty()
            || self.target_device_id().trim().is_empty()
        {
            return Err(ChildStorageCustodyAuthorityError::InvalidBinding);
        }
        if self.authority_generation() == 0 || self.session_generation() == 0 {
            return Err(ChildStorageCustodyAuthorityError::InvalidGeneration);
        }
        if !self.source.is_current() {
            return Err(ChildStorageCustodyAuthorityError::StaleOrRevoked);
        }
        if !self.source.allows(effect) {
            return Err(ChildStorageCustodyAuthorityError::EffectNotGranted);
        }
        Ok(())
    }

    pub(super) fn custody_input(
        &self,
        effect: StorageCustodyEffectKind,
    ) -> Option<ocentra_storage_custody_core::storage_custody::StorageCustodyInput> {
        self.source.custody_input(effect)
    }

    pub(super) fn allows_local_payload(&self, relative_path: &Path) -> bool {
        self.source.allows_local_payload(relative_path)
    }

    pub(super) fn is_current_with_generations(
        &self,
        authority_generation: u64,
        session_generation: u64,
    ) -> bool {
        authority_generation != 0
            && session_generation != 0
            && self.authority_generation() == authority_generation
            && self.session_generation() == session_generation
            && self.source.is_current()
    }

    pub(super) fn has_current_binding(&self) -> bool {
        !self.household_id().trim().is_empty()
            && !self.child_profile_id().trim().is_empty()
            && !self.target_device_id().trim().is_empty()
            && self.authority_generation() != 0
            && self.session_generation() != 0
            && self.source.is_current()
    }
}

pub(super) fn record_still_matches_authority(
    authority: &ChildStorageCustodyAuthorityHandle,
    record: &StorageCustodyEffectRecord,
) -> bool {
    authority.validate_for(record.effect_kind).is_ok()
        && authority.household_id() == record.household_id
        && authority.child_profile_id() == record.child_profile_id
        && authority.target_device_id() == record.target_device_id
        && authority.custody_input(record.effect_kind) == Some(record.custody_input)
        && match record.effect_kind {
            StorageCustodyEffectKind::LocalDelete => record
                .relative_path
                .as_deref()
                .is_some_and(|path| authority.allows_local_payload(Path::new(path))),
            _ => true,
        }
        && authority
            .is_current_with_generations(record.authority_generation, record.session_generation)
}
