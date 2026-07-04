mod counts;
mod normalize;
mod status_entry;
mod status_mapping;

use super::*;
use crate::adapter_capability_status::{
    NetworkAdapterCapabilityStatusEntry, NetworkAdapterCapabilityStatusError,
    NetworkAdapterCapabilityStatusState,
};

pub(crate) use counts::status_counts;
pub(crate) use normalize::{normalize_portal_ref, normalize_ref};
pub(crate) use status_entry::status_entry_from_platform_entry;
