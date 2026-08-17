use std::path::Path;

use ocentra_storage_custody_core::storage_custody::{
    StorageCustodyEffectKind, StorageCustodyInput,
};

use super::{ChildStorageCustodyAuthority, ManualRequiredChildStorageCustodyAuthority};

impl ChildStorageCustodyAuthority for ManualRequiredChildStorageCustodyAuthority {
    fn household_id(&self) -> &str {
        ""
    }

    fn child_profile_id(&self) -> &str {
        ""
    }

    fn target_device_id(&self) -> &str {
        ""
    }

    fn authority_generation(&self) -> u64 {
        0
    }

    fn session_generation(&self) -> u64 {
        0
    }

    fn is_current(&self) -> bool {
        false
    }

    fn allows(&self, _effect: StorageCustodyEffectKind) -> bool {
        false
    }

    fn custody_input(&self, _effect: StorageCustodyEffectKind) -> Option<StorageCustodyInput> {
        None
    }

    fn allows_local_payload(&self, _relative_path: &Path) -> bool {
        false
    }
}
