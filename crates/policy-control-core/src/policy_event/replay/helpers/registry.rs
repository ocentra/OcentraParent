#![forbid(unsafe_code)]

use ocentra_eventing::contract_registry::EventContractRegistry;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{EventNamespace, EventType, SchemaVersion};
use ocentra_eventing::topology::EventTopologyFamilyVariant;

const POLICY_EVENT_SCHEMA_VERSION_VALUE: u16 = 1;
const POLICY_EVENT_NAMESPACE_VALUE: &str = "policy";

pub(crate) fn policy_event_schema_version() -> Result<SchemaVersion, EventingError> {
    SchemaVersion::new(POLICY_EVENT_SCHEMA_VERSION_VALUE)
}

pub(crate) fn policy_event_family_namespace() -> Result<EventNamespace, EventingError> {
    EventNamespace::parse(POLICY_EVENT_NAMESPACE_VALUE)
}

pub(crate) fn policy_event_family_variants(
) -> Result<Vec<EventTopologyFamilyVariant>, EventingError> {
    let family = policy_event_family_namespace()?;
    crate::policy_event::policy_event_kinds()
        .iter()
        .copied()
        .map(|kind| {
            Ok(EventTopologyFamilyVariant {
                family: family.clone(),
                event_type: EventType::parse(kind.event_type_name())?,
            })
        })
        .collect()
}

pub(crate) fn policy_event_contract_registry() -> Result<EventContractRegistry, EventingError> {
    let mut registry = EventContractRegistry::new();
    for kind in crate::policy_event::policy_event_kinds().iter().copied() {
        let event = super::sample::sample_policy_event(kind)?;
        registry.register_event(&event)?;
    }
    Ok(registry)
}
