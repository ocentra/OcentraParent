#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;

use crate::policy_source::{
    ParentPolicySourceDocument, PolicyDocumentCompatibilityState, PolicySourceCompatibilityReport,
    PolicyVersion,
};

pub(super) fn assess_policy_source_compatibility(
    source: &ParentPolicySourceDocument,
    supported_schema_version: SchemaVersion,
    minimum_supported_policy_version: PolicyVersion,
) -> Result<PolicySourceCompatibilityReport, EventingError> {
    let schema_state = schema_compatibility_state(
        source.schema_version.value(),
        supported_schema_version.value(),
    );
    let policy_version_state =
        policy_version_compatibility_state(source.policy_version, minimum_supported_policy_version);

    Ok(PolicySourceCompatibilityReport {
        source_schema_version: source.schema_version,
        supported_schema_version,
        source_policy_version: source.policy_version,
        minimum_supported_policy_version,
        schema_state,
        policy_version_state,
    })
}

fn schema_compatibility_state(
    source_schema_version: u16,
    supported_schema_version: u16,
) -> PolicyDocumentCompatibilityState {
    if source_schema_version == supported_schema_version {
        PolicyDocumentCompatibilityState::Compatible
    } else if source_schema_version < supported_schema_version {
        PolicyDocumentCompatibilityState::MigrationRequired
    } else {
        PolicyDocumentCompatibilityState::Unsupported
    }
}

fn policy_version_compatibility_state(
    source_policy_version: PolicyVersion,
    minimum_supported_policy_version: PolicyVersion,
) -> PolicyDocumentCompatibilityState {
    if source_policy_version.value() < minimum_supported_policy_version.value() {
        PolicyDocumentCompatibilityState::MigrationRequired
    } else {
        PolicyDocumentCompatibilityState::Compatible
    }
}
